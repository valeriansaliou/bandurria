// Bandurria
//
// Lightweight comment system for static websites
// Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use log::{self, Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct ConfigLogger;

impl Log for ConfigLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("({}) - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl ConfigLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_max_level(level);
        log::set_logger(&ConfigLogger)
    }
}
