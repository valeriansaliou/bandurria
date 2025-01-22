// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::path::PathBuf;

use hex;
use uuid::Uuid;

pub fn server_log_level() -> String {
    "error".to_string()
}

pub fn server_inet() -> SocketAddr {
    "[::1]:8080".parse().unwrap()
}

pub fn assets_path() -> PathBuf {
    PathBuf::from("./res/assets/")
}

pub fn email_smtp_server_port() -> u16 {
    587
}

pub fn email_smtp_server_starttls() -> bool {
    true
}

pub fn email_smtp_server_tls() -> bool {
    false
}

pub fn email_identity_from_name() -> String {
    return "Comments".to_string();
}

pub fn security_secret_key() -> String {
    // While we recommend that the user pass their own secret key in the \
    //   configuration, thus ensuring that all signed URLs persist across \
    //   restarts, we still provide a convenient way to start Bandurria with \
    //   a random UUIDv4-based secret key if none is provided.
    hex::encode(Uuid::new_v4().as_bytes())
}
