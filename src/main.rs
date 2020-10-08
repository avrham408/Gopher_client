#![allow(unused_variables)]

#[macro_use]

pub mod config;

use client::networking;
use log::*;

fn main() {
    let _conf = config::Config::from_env();
    let v = networking::get_ip_address("gopher.club".into(), 70).unwrap();
    let connection = networking::open_connection(&v[0]);
    info!("adar");
}
