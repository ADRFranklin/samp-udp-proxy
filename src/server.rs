/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           servers.rs
    @Project        SA-MP Proxy
    @Created        20th March 2019
    @Weburl         https://sanandreasgaming.com
                    https://burgershot.gg/member.php?action=profile&uid=5

    - README -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/README.md

    - LICENSE -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/LICENSE
*/

// --
//  Modules
// --

pub use crate::config::Backend;

// --
//  Main
// --

#[derive(Debug)]
pub struct Server {
    pub ip: String,
    pub port: u16,
    pub cache: Cache,
}

#[derive(Debug)]
pub struct Cache {
    pub hostname: String,
    pub password: u8,
    pub max_players: u16,
    pub players: Vec<Player>,
    pub gamemode: String,
    pub language: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub ping: u32,
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub value: String,
}

impl Server {
    pub fn new(config: Backend) -> Server {
        return Server {
            ip: config.ip,
            port: config.port,
            cache: Cache::default(),
        };
    }
}

impl Default for Cache {
    fn default() -> Cache {
        return Cache {
            hostname: "Unknown Hostname".to_string(),
            password: 0,
            max_players: 0,
            players: vec![],
            gamemode: "Unknown Gamemode".to_string(),
            language: "Unknown Language".to_string(),
            rules: vec![],
        };
    }
}

impl Player {
    pub fn new(name: String, score: u32, ping: u32) -> Player {
        return Player {
            name: name,
            score: score,
            ping: ping,
        };
    }
}

impl Rule {
    pub fn new(name: String, value: String) -> Rule {
        return Rule {
            name: name,
            value: value,
        };
    }
}
