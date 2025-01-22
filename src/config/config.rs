// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::net::SocketAddr;
use std::path::PathBuf;

use super::defaults;

#[derive(Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub assets: ConfigAssets,
    pub database: ConfigDatabase,
    pub email: ConfigEmail,
    pub site: ConfigSite,
    pub security: ConfigSecurity,
    pub i18n: ConfigI18N,
}

#[derive(Deserialize)]
pub struct ConfigServer {
    #[serde(default = "defaults::server_log_level")]
    pub log_level: String,

    #[serde(default = "defaults::server_inet")]
    pub inet: SocketAddr,
}

#[derive(Deserialize)]
pub struct ConfigAssets {
    #[serde(default = "defaults::assets_path")]
    pub path: PathBuf,
}

#[derive(Deserialize)]
pub struct ConfigDatabase {
    pub mysql: ConfigDatabaseMySQL,
}

#[derive(Deserialize)]
pub struct ConfigDatabaseMySQL {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct ConfigEmail {
    pub smtp: ConfigEmailSMTP,
    pub identity: ConfigEmailIdentity,
}

#[derive(Deserialize)]
pub struct ConfigEmailSMTP {
    pub server_host: String,

    #[serde(default = "defaults::email_smtp_server_port")]
    pub server_port: u16,

    #[serde(default = "defaults::email_smtp_server_starttls")]
    pub server_starttls: bool,

    #[serde(default = "defaults::email_smtp_server_tls")]
    pub server_tls: bool,

    pub auth_user: Option<String>,
    pub auth_password: Option<String>,
}

#[derive(Deserialize)]
pub struct ConfigEmailIdentity {
    #[serde(default = "defaults::email_identity_from_name")]
    pub from_name: String,

    pub from_email: String,
}

#[derive(Deserialize)]
pub struct ConfigSite {
    pub name: String,
    pub site_url: String,
    pub comments_url: String,
    pub admin_emails: Vec<String>,
}

#[derive(Deserialize)]
pub struct ConfigSecurity {
    #[serde(default = "defaults::security_secret_key")]
    pub secret_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigI18N {
    #[serde(default = "defaults::i18n_field_write_your_comment")]
    pub field_write_your_comment: String,

    #[serde(default = "defaults::i18n_field_whats_your_name")]
    pub field_whats_your_name: String,

    #[serde(default = "defaults::i18n_field_whats_your_email")]
    pub field_whats_your_email: String,

    #[serde(default = "defaults::i18n_button_post_comment")]
    pub button_post_comment: String,

    #[serde(default = "defaults::i18n_button_reply")]
    pub button_reply: String,

    #[serde(default = "defaults::i18n_label_leave_a_comment")]
    pub label_leave_a_comment: String,

    #[serde(default = "defaults::i18n_banner_presubmit")]
    pub banner_presubmit: String,

    #[serde(default = "defaults::i18n_banner_submitted_important")]
    pub banner_submitted_important: String,

    #[serde(default = "defaults::i18n_banner_submitted_notice")]
    pub banner_submitted_notice: String,

    #[serde(default = "defaults::i18n_banner_submiterror")]
    pub banner_submiterror: String,
}
