#![allow(unused_variables)]

#[macro_use]

pub mod config;

use log::*;

fn main() {
    let _conf = config::Config::from_env();

    info!("adar");
}
