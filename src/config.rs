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

// libs
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub proxy: Proxy,
    pub servers: Servers,
}

#[derive(Deserialize, Debug)]
pub struct Proxy {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Servers {
    pub ip: String,
    pub port: u16,
}

impl Config {
    pub fn parse(path: String) -> Result<Config, Box<dyn Error>> {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}
