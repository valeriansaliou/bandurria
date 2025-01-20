// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod config;
mod managers;
mod routes;

use std::ops::Deref;
use std::str::FromStr;

use clap::{Arg, Command};
use config::config::Config;
use config::logger::ConfigLogger;
use config::reader::ConfigReader;
use log::LevelFilter;
use managers::http;

struct AppArgs {
    config: String,
}

lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
    static ref APP_CONF: Config = ConfigReader::make();
}

fn make_app_args() -> AppArgs {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to configuration file")
                .default_value("./config.cfg"),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs {
        config: matches
            .get_one::<String>("config")
            .expect("invalid config value")
            .to_owned(),
    }
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (APP_ARGS.deref(), APP_CONF.deref());
}

#[rocket::main]
async fn main() {
    // Initialize shared logger
    let _logger = ConfigLogger::init(
        LevelFilter::from_str(&APP_CONF.server.log_level).expect("invalid log level"),
    );

    info!("starting up");

    // Ensure all states are bound
    ensure_states();

    // Start HTTP server
    match http::bootstrap().await.launch().await {
        Ok(_) => log::info!("stopped"),
        Err(_) => log::error!("could not start"),
    };

    error!("could not start");
}
