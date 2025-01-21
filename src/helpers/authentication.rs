// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::normalize;
use crate::APP_CONF;

pub fn check_email_is_admin(raw_email: &str) -> bool {
    let email = normalize::email(raw_email);

    APP_CONF.site.admin_emails.contains(&email)
}
