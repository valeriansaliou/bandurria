// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::path::PathBuf;

use hex;
use uuid::Uuid;

use crate::helpers::mint::{MintDifficulty, MintSolutions};

/* [server] */

pub fn server_log_level() -> String {
    "error".to_string()
}

pub fn server_inet() -> SocketAddr {
    "[::1]:8080".parse().unwrap()
}

/* [assets] */

pub fn assets_path() -> PathBuf {
    PathBuf::from("./res/assets/")
}

/* [email] */

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

/* [security] */

pub fn security_secret_key() -> String {
    // While we recommend that the user pass their own secret key in the \
    //   configuration, thus ensuring that all signed URLs persist across \
    //   restarts, we still provide a convenient way to start Bandurria with \
    //   a random UUIDv4-based secret key if none is provided.
    hex::encode(Uuid::new_v4().as_bytes())
}

pub fn security_check_pages_exist() -> bool {
    false
}

/* [antispam] */

pub fn antispam_difficulty() -> MintDifficulty {
    17
}

pub fn antispam_problems_parallel() -> MintSolutions {
    10
}

pub fn antispam_solutions_require() -> MintSolutions {
    6
}

/* [avatar] */

pub fn avatar_gravatar() -> bool {
    false
}

pub fn avatar_size_pixels() -> u16 {
    20
}

pub fn avatar_scale_factor() -> u8 {
    3
}

/* [i18n] */

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

pub fn i18n_label_subscribe_replies() -> String {
    "I want to get notified over email when the site owner replies.".into()
}

pub fn i18n_banner_presubmit() -> String {
    "Your email is only stored if you opt-in to receive replies to your comment.".into()
}

pub fn i18n_banner_submitting() -> String {
    "Sending and proving you are not a bot. This might take a few seconds...".into()
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
