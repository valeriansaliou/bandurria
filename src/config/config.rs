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
    pub site: ConfigSite,
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
pub struct ConfigSite {
    pub base_url: String,
    pub admin_emails: Vec<String>,
}
