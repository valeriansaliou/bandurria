// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use hex;
use rocket::http::Status;
use sha2::{Digest, Sha256};

pub fn page_url(page: &str) -> Result<String, Status> {
    // Step 1: Ensure there is no query string (take what comes before '?')
    let page_with_no_querystring = page.split("?").into_iter().next().unwrap_or("");

    // Step 2: ensure there are no double slashes or empty segments in path
    let page_with_clean_segments = page_with_no_querystring
        .split("/")
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<&str>>()
        .join("/");

    // Step 3: finalize normalized page (with leading + trailing slashes)
    let normalized_page = format!("/{}/", page_with_clean_segments.to_lowercase());

    Ok(normalized_page)
}

pub fn email(raw_email: &str) -> String {
    raw_email.to_lowercase()
}

pub fn email_hash(raw_email: &str) -> String {
    // Normalize email
    let email_normalized = email(raw_email);

    let mut hasher = Sha256::new();

    hasher.update(&email_normalized);

    hex::encode_upper(hasher.finalize())
}
