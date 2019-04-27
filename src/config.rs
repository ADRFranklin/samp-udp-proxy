/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           config.rs
    @Project        SA-MP Proxy
    @Created        20th March 2019
    @Weburl         https://sanandreasgaming.com
                    https://burgershot.gg/member.php?action=profile&uid=5

    - README -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/README.md

    - LICENCE -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/LICENSE
*/

// crates
extern crate serde;
extern crate toml;

// libs
use self::serde::*;
use std::error;
use std::fs::File;
use std::io::Read;

// --
//  Structs
// --

#[derive(Deserialize, Debug)]
pub struct Config {
    pub proxy: Proxy,
    pub servers: Vec<Servers>,
}

#[derive(Deserialize, Debug)]
struct Proxy {
    ip: String,
    port: Option<u16>,
}

#[derive(Deserialize, Debug)]
struct Servers {
    name: String,
    ip: String,
    port: Option<u16>,
}

// --
//  Load Config
// --

pub fn parse(path: String) -> Result<Config, Box<dyn error::Error>> {
    let mut config_path = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to load the config file. {}", e),
    };

    let mut contents = String::new();
    match config_path.read_to_string(&mut contents) {
        Ok(c) => c,
        Err(e) => panic!("Failed to read config file. {}", e),
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => panic!("Failed to parse config file. {}", e),
    };

    return Ok(config);
}
