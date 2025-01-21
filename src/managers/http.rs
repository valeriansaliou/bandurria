// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::path::PathBuf;

use rocket::config::Ident;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::{self, Config};
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};
use rocket_dyn_templates::Template;

use crate::routes::{api, page};
use crate::APP_CONF;

#[derive(Database)]
#[database("mysql_bandurria")]
pub struct Db(sqlx::MySqlPool);

pub type DbConn = Connection<Db>;

fn configure() -> Figment {
    let config = Config {
        address: APP_CONF.server.inet.ip(),
        port: APP_CONF.server.inet.port(),
        workers: 1,
        max_blocking: 8,
        ident: Ident::try_new("Bandurria").unwrap(),
        limits: Limits::default().limit("json", 256.kibibytes()),
        ..Config::default()
    };

    Figment::from(config)
        .merge(("template_dir", assets_path("templates")))
        .merge((
            "databases.mysql_bandurria.url",
            &APP_CONF.database.mysql.uri,
        ))
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
    rocket::custom(configure())
        .attach(Db::init())
        .attach(Template::fairing())
        .mount("/api", rocket::routes![api::post_comment])
        .mount("/page", rocket::routes![page::get_comments])
        .mount("/assets", FileServer::from(assets_path("public")))
        .mount("/dev", FileServer::from(assets_path("dev")))
}
