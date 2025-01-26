// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

// See: https://therootcompany.com/blog/http-hashcash/

use std::collections::HashSet;
use std::time::{Duration, SystemTime};

use base64_url;
use regex::Regex;
use sha2::{Digest, Sha256};

use super::authentication;
use crate::APP_CONF;

pub type MintSolutions = u8;
pub type MintDifficulty = u8;

type MintIndex = u8;
type MintTimestamp = u64;

const VALIDITY: Duration = Duration::from_secs(300);
const ALGORITHM: &'static str = "SHA-256";

lazy_static! {
    static ref SOLUTION_REGEX: Regex =
        Regex::new(r"^H:([0-9]+):([0-9]+):([^:/]+)/([0-9]+):([^:]+):([^:]+):([^:]+)$").unwrap();
}

pub fn challenge(comment_id: &str) -> Result<(Vec<String>, MintDifficulty, MintSolutions), ()> {
    // Generate expire time (the challenge has a validity period)
    let expire_at_time = SystemTime::now() + VALIDITY;

    let expire_at = expire_at_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .or(Err(()))?
        .as_secs();

    // Generate all problem strings
    let mut problems = Vec::with_capacity(APP_CONF.antispam.problems_parallel as usize);

    for index in 0..APP_CONF.antispam.problems_parallel {
        problems.push(make_problem(
            APP_CONF.antispam.difficulty,
            index,
            comment_id,
            expire_at,
        )?);
    }

    info!("generated mint challenge problems: {:?}", problems);

    Ok((
        problems,
        APP_CONF.antispam.difficulty,
        APP_CONF.antispam.solutions_require,
    ))
}

pub fn verify(reference_comment_id: &str, solutions: &[String]) -> Result<bool, ()> {
    let now_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .or(Err(()))?
        .as_secs();

    info!(
        "verifying mint for comment: {}, with solutions: {:?}",
        reference_comment_id, solutions
    );

    // 1. Extract solution contents (return early if a solution is invalid)
    let mut vectors_to_verify: Vec<(MintDifficulty, &str)> = Vec::new();
    let mut appended_indexes: HashSet<MintIndex> = HashSet::new();

    for solution in solutions {
        if let Some(matches) = SOLUTION_REGEX.captures(solution) {
            let (_, [difficulty, expire_at, comment_id, index, nonce, algorithm, _]) =
                matches.extract();

            // 1.1. Do not append when nonce does not match content
            // Notice: this is a crucial step that guards against vector \
            //   data tampering.
            match vector_to_nonce(comment_id, difficulty, index, expire_at) {
                Ok(reference_nonce) => {
                    if nonce != reference_nonce {
                        warn!(
                            "[verify] could not match vector nonces: {} <> {}",
                            reference_nonce, nonce
                        );

                        continue;
                    }
                }
                Err(_) => {
                    error!("[verify] could not convert vector to nonce");

                    continue;
                }
            }

            // 1.2. Do not append when solution is expired
            match expire_at.parse::<MintTimestamp>() {
                Ok(expire_at) => {
                    // Expired?
                    if now_timestamp >= expire_at {
                        warn!("[verify] got expired solution at: {expire_at}");

                        continue;
                    }
                }
                Err(err) => {
                    error!("[verify] could not parse solution expire: {err}");

                    // Consider expired
                    continue;
                }
            }

            // 1.3. Do not append when comment identifier do not match
            if comment_id != reference_comment_id {
                warn!(
                    "[verify] got comment mismatch in solution: {} <> {}",
                    reference_comment_id, comment_id
                );

                continue;
            }

            // 1.4. Do not append if algorithm is not supported
            if algorithm != ALGORITHM {
                warn!("[verify] got unsupported solution algorithm: {algorithm}");

                continue;
            }

            // 1.5. Do not append if vector index has already been appended
            // Notice: this prevents against attacks where the same vector \
            //   would be submitted enough times to satisfy the \
            //   SOLUTIONS_REQUIRED parameter, although those vectors would \
            //   all hold the same solution (since they have the same index).
            match index.parse::<MintIndex>() {
                Ok(index) => {
                    // Already inserted? (if this insert is falsy)
                    if !appended_indexes.insert(index) {
                        warn!("[verify] got duplicate solution index at: {index}");

                        continue;
                    }
                }
                Err(err) => {
                    error!("[verify] could not parse solution index: {err}");

                    // Consider index as invalid
                    continue;
                }
            }

            // 1.6. Parse difficulty to number and append vector
            match difficulty.parse::<MintDifficulty>() {
                Ok(difficulty) => {
                    vectors_to_verify.push((difficulty, solution));
                }
                Err(err) => {
                    error!("[verify] could not parse solution difficulty: {err}");

                    // Consider difficulty as invalid
                    continue;
                }
            }
        } else {
            warn!("[verify] got invalid solution format: {solution}");
        }
    }

    // 2. Verify each solution (count)
    let mut verified_solutions = 0;

    for vector_to_verify in vectors_to_verify {
        let (is_verified, leading_zeroes) = verify_solution(vector_to_verify.0, vector_to_verify.1);

        // Solution verified? Increment count of verified solutions
        if is_verified {
            debug!(
                "[verify] verified solution vector: {} ({} leading zeroes)",
                vector_to_verify.1, leading_zeroes
            );

            verified_solutions += 1;
        } else {
            warn!(
                "[verify] could not verify solution vector: {} ({} leading zeroes)",
                vector_to_verify.1, leading_zeroes
            );
        }
    }

    // 3. Ensure we have at least SOLUTIONS_REQUIRED verified solutions
    Ok(verified_solutions >= APP_CONF.antispam.solutions_require)
}

fn make_problem(
    difficulty: MintDifficulty,
    index: MintIndex,
    comment_id: &str,
    expire_at: MintTimestamp,
) -> Result<String, ()> {
    // Nonce is actually the signature of the Hashcash payload contents, so \
    //   that we can guard against double-spending (since the comment \
    //   identifier can only be redeemed once), and also expiration to prevent \
    //   attacks modes where a spam bot would acquire a lot of challenges, \
    //   compute the solutions in advance over a long period of time, and then \
    //   spend those solutions very quickly during a short-lived but \
    //   high-scale spam attack. Lastly, the problem index must also be \
    //   included in the signature, since each problem is independant from \
    //   each other.
    let nonce = vector_to_nonce(
        comment_id,
        &difficulty.to_string(),
        &index.to_string(),
        &expire_at.to_string(),
    )?;

    Ok(format!(
        "H:{difficulty}:{expire_at}:{comment_id}/{index}:{nonce}:{ALGORITHM}"
    ))
}

fn verify_solution(
    required_difficulty: MintDifficulty,
    solution_string: &str,
) -> (bool, MintDifficulty) {
    let mut hasher = Sha256::new();

    hasher.update(solution_string);

    let solution_hash = hasher.finalize();

    // Count leading zero bytes in the hash
    let mut leading_zeroes_count: u32 = 0;
    let leading_zeroes_overflow_cap = MintDifficulty::MAX as u32;

    for solution_hash_byte in solution_hash {
        let byte_leading_zeroes = solution_hash_byte.leading_zeros();

        // Increment by the number of found zeroes.
        leading_zeroes_count += byte_leading_zeroes;

        // Not full leading zeroes found in this byte? Stop count here
        if byte_leading_zeroes < u8::BITS {
            break;
        }

        // Guard against integer overflows, since we will convert back the \
        //   counter to a difficulty type right after
        if leading_zeroes_count >= leading_zeroes_overflow_cap {
            break;
        }
    }

    // Return whether we had enough leading zeroes or not (we require at \
    //   least 'required_difficulty' leading zeroes), more is accepted but not \
    //   necessary.
    (
        leading_zeroes_count as MintDifficulty >= required_difficulty,
        leading_zeroes_count as MintDifficulty,
    )
}

fn vector_to_nonce(
    comment_id: &str,
    difficulty_str: &str,
    index_str: &str,
    expire_at_str: &str,
) -> Result<String, ()> {
    let nonce_payload = format!("{difficulty_str}>{index_str}>{comment_id}>{expire_at_str}");
    let nonce_signature = authentication::sign_payload_bytes(&nonce_payload)?;

    Ok(base64_url::encode(&nonce_signature))
}
