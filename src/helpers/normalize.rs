// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket::http::Status;
use sha2::{Digest, Sha256};

pub fn page_url(page: &str) -> Result<String, Status> {
    // TODO: we might want to normalize in a better way here
    Ok(page.to_lowercase())
}

pub fn email(raw_email: &str) -> String {
    raw_email.to_lowercase()
}

pub fn email_hash(raw_email: &str) -> String {
    // Normalize email
    let email_normalized = email(raw_email);

    let mut hasher = Sha256::new();

    hasher.update(&email_normalized);

    format!("{:X}", hasher.finalize())
}
