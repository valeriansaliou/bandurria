// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::APP_CONF;

type HmacSha256 = Hmac<Sha256>;

pub fn sign_payload(payload: &str) -> Result<String, ()> {
    let mut hmac =
        HmacSha256::new_from_slice(APP_CONF.security.secret_key.as_bytes()).or(Err(()))?;

    hmac.update(payload.as_bytes());

    let signature_bytes = hmac.finalize().into_bytes();

    Ok(hex::encode(signature_bytes))
}

pub fn verify_payload(payload: &str, signature: &str) -> bool {
    if let Ok(reference_signature) = sign_payload(payload) {
        reference_signature == signature
    } else {
        false
    }
}
