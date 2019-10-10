/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           lib.rs
    @Project        SA-MP Proxy
    @Created        20th March 2019
    @Weburl         https://sanandreasgaming.com
                    https://burgershot.gg/member.php?action=profile&uid=5

    - README -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/README.md

    - LICENCE -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/LICENSE
*/

extern crate byteorder;

mod config;
mod eventloop;
mod packet;
mod proxy;
mod query;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use crate::config::Config;
use crate::packet::Packet;
use crate::proxy::Proxy;

const CONFIG: &str = "config.toml";
const THREAD_COUNT: u32 = 4;

pub fn main() {
    let config = match Config::parse(CONFIG.into()) {
        Ok(c) => c,
        Err(e) => {
            panic!("Failed to parse the config file: {}", e);
        }
    };

    let address = &format!("{}:{}", config.proxy.ip, config.proxy.port);
    start(address, THREAD_COUNT);
}

pub fn start(address: &str, threads: u32) {
    let (sender, receiver): (Sender<Packet>, Receiver<Packet>) = mpsc::channel();
    let proxy = Proxy::new(address, threads, sender);
    let _ = proxy.spawn_threads();
    eventloop::run(proxy, receiver);
}
