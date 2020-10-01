#[macro_use]
pub mod config;

use client::networking;
use log::*;

fn main() {
    config::Config::from_env();
    info!("run end");
}
