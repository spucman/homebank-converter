#[macro_use]
extern crate log;

use log::LevelFilter;

mod bank;
mod cli;
mod config;
mod data;

fn main() {
    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    log::set_max_level(LevelFilter::Debug);

    info!("some information log");
    warn!("some warning log");
    error!("some error log");
    info!("some information log");
}
