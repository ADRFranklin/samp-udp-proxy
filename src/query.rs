/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           query.rs
    @Project        SA-MP Proxy
    @Created        20th March 2019
    @Weburl         https://sanandreasgaming.com
                    https://burgershot.gg/member.php?action=profile&uid=5

    - README -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/README.md

    - LICENSE -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/LICENSE
*/

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::io::Write;

pub use crate::packet::Packet;
pub use crate::server::{Cache, Player, Rule, Server};

// --
//  SAMP Query
// --

pub struct Query {
    pub packet: Packet,
    pub cache: Cache,
}

impl Query {
    pub fn new(packet: Packet, cache: Cache) -> Query {
        return Query {
            packet: packet,
            cache: cache,
        };
    }

    pub fn write_query_data(&self, writer: &mut std::io::Cursor<Vec<u8>>) {
        writer.write_u32::<BigEndian>(1396788560 as u32).expect("");
        writer.write_u32::<BigEndian>(self.packet.ip).expect("");
        writer.write_u16::<BigEndian>(self.packet.port).expect("");
        writer.write_u8(self.packet.data[0]).expect("");
    }

    pub fn send_information(&self, writer: &mut std::io::Cursor<Vec<u8>>) {
        self.write_query_data(writer);

        writer.write_u8(self.cache.password).expect("");
        writer
            .write_u16::<LittleEndian>((self.cache.players).len() as u16)
            .expect("");
        writer
            .write_u16::<LittleEndian>(self.cache.max_players)
            .expect("");
        writer
            .write_u32::<LittleEndian>(self.cache.hostname.len() as u32)
            .expect("");
        writer.write(self.cache.hostname.as_bytes()).expect("");
        writer
            .write_u32::<LittleEndian>(self.cache.gamemode.len() as u32)
            .expect("");
        writer.write(self.cache.gamemode.as_bytes()).expect("");
        writer
            .write_u32::<LittleEndian>(self.cache.language.len() as u32)
            .expect("");
        writer.write(self.cache.language.as_bytes()).expect("");
    }

    pub fn send_rules(&self, writer: &mut std::io::Cursor<Vec<u8>>) {
        self.write_query_data(writer);

        writer
            .write_u16::<LittleEndian>(self.cache.rules.len() as u16)
            .expect("");

        for rule in self.cache.rules.iter() {
            writer.write_u8(rule.name.len() as u8).expect("");
            writer.write(rule.name.as_bytes()).expect("");
            writer.write_u8(rule.value.len() as u8).expect("");
            writer.write(rule.value.as_bytes()).expect("");
        }
    }

    pub fn send_players(&self, writer: &mut std::io::Cursor<Vec<u8>>) {
        self.write_query_data(writer);

        writer
            .write_u16::<LittleEndian>(self.cache.players.len() as u16)
            .expect("");

        for player in self.cache.players.iter() {
            writer.write_u8(player.name.len() as u8).expect("");
            writer.write(player.name.as_bytes()).expect("");
            writer.write_u8(player.score as u8).expect("");
            writer.write_u32::<LittleEndian>(player.score).expect("");
        }
    }

    pub fn send_detailed_players(&self, writer: &mut std::io::Cursor<Vec<u8>>) {
        self.write_query_data(writer);

        writer
            .write_u16::<LittleEndian>(self.cache.players.len() as u16)
            .expect("");
        for player in self.cache.players.iter() {
            writer.write_u8(player.name.len() as u8).expect("");
            writer.write(player.name.as_bytes()).expect("");
            writer.write_u8(player.score as u8).expect("");
            writer.write_u32::<LittleEndian>(player.score).expect("");
            writer.write_u32::<LittleEndian>(player.ping).expect("");
        }
    }

    pub fn send_ping(&self, writer: &mut std::io::Cursor<Vec<u8>>) {
        self.write_query_data(writer);

        writer.write_u8(self.packet.data[1]).expect("");
        writer.write_u8(self.packet.data[2]).expect("");
        writer.write_u8(self.packet.data[3]).expect("");
        writer.write_u8(self.packet.data[4]).expect("");
    }
}
