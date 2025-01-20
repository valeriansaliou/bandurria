// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::path::PathBuf;

use rocket::config::Ident;
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::{self, Config};
use rocket_dyn_templates::Template;

use crate::routes::{api, page};
use crate::APP_CONF;

fn configure() -> Figment {
    let config = Config {
        address: APP_CONF.server.inet.ip(),
        port: APP_CONF.server.inet.port(),
        workers: 1,
        max_blocking: 8,
        ident: Ident::try_new("Bandurria").unwrap(),
        ..Config::default()
    };

    Figment::from(config).merge(("template_dir", assets_path("templates")))
}

fn assets_path(kind: &str) -> PathBuf {
    APP_CONF
        .assets
        .path
        .canonicalize()
        .unwrap()
        .join(kind)
        .to_path_buf()
}

pub async fn bootstrap() -> rocket::Rocket<rocket::Build> {
    // TODO: dev not supposed to be here in release mode
    rocket::custom(configure())
        .mount("/api", rocket::routes![api::get_base])
        .mount("/page", rocket::routes![page::get_comments])
        .mount("/assets", FileServer::from(assets_path("public")))
        .mount("/dev", FileServer::from(assets_path("dev")))
        .attach(Template::fairing())
}
