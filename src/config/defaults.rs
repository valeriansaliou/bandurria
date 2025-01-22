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
    "Comments".into()
}

pub fn security_secret_key() -> String {
    // While we recommend that the user pass their own secret key in the \
    //   configuration, thus ensuring that all signed URLs persist across \
    //   restarts, we still provide a convenient way to start Bandurria with \
    //   a random UUIDv4-based secret key if none is provided.
    hex::encode(Uuid::new_v4().as_bytes())
}

pub fn i18n_field_write_your_comment() -> String {
    "Write your comment...".into()
}

pub fn i18n_field_whats_your_name() -> String {
    "What's your name?".into()
}

pub fn i18n_field_whats_your_email() -> String {
    "Enter your email".into()
}

pub fn i18n_button_post_comment() -> String {
    "Post comment".into()
}

pub fn i18n_button_reply() -> String {
    "Reply".into()
}

pub fn i18n_label_leave_a_comment() -> String {
    "Leave a comment:".into()
}

pub fn i18n_banner_presubmit() -> String {
    "Your email is only used to check you are not a bot. It will not be stored.".into()
}

pub fn i18n_banner_submitted_important() -> String {
    "Your comment has been submitted.".into()
}

pub fn i18n_banner_submitted_notice() -> String {
    "It will appear here after it gets accepted by moderation.".into()
}

pub fn i18n_banner_submiterror() -> String {
    "Your comment could not be submitted. Mind try again?".into()
}
