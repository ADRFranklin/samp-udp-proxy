/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           server.rs
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

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::io::Write;
use std::net::UdpSocket;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

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

#[derive(Debug, Clone)]
pub struct Cache {
    pub hostname: String,
    pub password: u8,
    pub max_players: u16,
    pub players: Vec<Player>,
    pub gamemode: String,
    pub language: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub ping: u32,
}

#[derive(Debug, Clone)]
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

    pub fn write_query_header(&self) -> Cursor<Vec<u8>> {
        let mut writer = Cursor::new(vec![]);
        writer.write("SAMP".as_bytes()).unwrap();
        writer.write(self.ip.as_bytes()).unwrap();
        writer.write_u16::<LittleEndian>(self.port).unwrap();
        writer
    }

    pub fn get_info(&self, writer: &mut Cursor<Vec<u8>>) {
        writer.write_u8("i".as_bytes()[0]).unwrap();
    }

    pub fn get_rules(&self, writer: &mut Cursor<Vec<u8>>) {
        writer.write_u8("r".as_bytes()[0]).unwrap();
    }

    pub fn get_players(&self, writer: &mut Cursor<Vec<u8>>) {
        writer.write_u8("d".as_bytes()[0]).unwrap();
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
