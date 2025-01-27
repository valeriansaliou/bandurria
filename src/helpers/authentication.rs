// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use hex;
use hmac::{Hmac, Mac};
use rocket::http::Status;
use sha2::Sha256;

use super::normalize;
use crate::APP_CONF;

type HmacSha256 = Hmac<Sha256>;

lazy_static! {
    static ref ADMIN_EMAIL_HASHES: Vec<String> = APP_CONF
        .site
        .admin_emails
        .iter()
        .map(|admin_email| normalize::email_hash(&admin_email))
        .collect();
}

pub fn check_email_hash_is_admin(email_hash: &String) -> bool {
    ADMIN_EMAIL_HASHES.contains(email_hash)
}

pub fn sign_payload_bytes(payload: &str) -> Result<Vec<u8>, ()> {
    let mut hmac =
        HmacSha256::new_from_slice(APP_CONF.security.secret_key.as_bytes()).or(Err(()))?;

    hmac.update(payload.as_bytes());

    Ok(hmac.finalize().into_bytes().to_vec())
}

pub fn sign_payload(payload: &str) -> Result<String, ()> {
    Ok(hex::encode(sign_payload_bytes(payload)?))
}

pub fn generate_challenge_attestation(page: &str, comment_id: &str) -> Result<String, Status> {
    let page_url = normalize::page_url(page)?;

    return sign_payload(&format!("{page_url}/{comment_id}")).or(Err(Status::UnprocessableEntity));
}

pub fn verify_challenge_attestation(page: &str, comment_id: &str, attestation: &str) -> bool {
    if let Ok(reference_attestation) = generate_challenge_attestation(page, comment_id) {
        reference_attestation == attestation
    } else {
        false
    }
}

pub fn generate_admin_comment_signature(action: &str, comment_id: &str) -> Result<String, Status> {
    return sign_payload(&format!("admin/{action}/{comment_id}"))
        .or(Err(Status::UnprocessableEntity));
}

pub fn verify_admin_comment_signature(action: &str, comment_id: &str, signature: &str) -> bool {
    if let Ok(reference_signature) = generate_admin_comment_signature(action, comment_id) {
        reference_signature == signature
    } else {
        false
    }
}
