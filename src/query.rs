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

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::io::Write;

pub use crate::server::{Cache, Player, Rule, Server};

// --
//  SAMP Query
// --

#[derive(Clone)]
pub struct QueryPacket {
    pub id: u32,
    pub ip: u32,
    pub port: u16,
    pub opcode: u8,
    pub data: Vec<u8>,
}

impl QueryPacket {
    pub fn new(id: u32, ip: u32, port: u16, opcode: u8, data: Vec<u8>) -> QueryPacket {
        return QueryPacket {
            id,
            ip,
            port,
            opcode,
            data,
        };
    }
}

pub fn parse_query_packet(mut buffer: &[u8]) -> QueryPacket {
    let id = buffer.read_u32::<BigEndian>().unwrap();
    let ip = buffer.read_u32::<BigEndian>().unwrap();
    let port = buffer.read_u16::<LittleEndian>().unwrap();
    let opcode = buffer.read_u8().unwrap();
    let data = buffer.to_vec();

    return QueryPacket::new(id, ip, port, opcode, data);
}

pub fn is_query_packet(id: u32) -> bool {
    id == 1396788560
}

pub fn write_query_header(packet: QueryPacket) -> std::io::Cursor<Vec<u8>> {
    let mut writer = Cursor::new(vec![]);
    writer.write_u32::<BigEndian>(packet.id).expect("");
    writer.write_u32::<BigEndian>(packet.ip).expect("");
    writer.write_u16::<LittleEndian>(packet.port).expect("");
    writer.write_u8(packet.opcode).expect("");
    writer
}

pub fn send_information(writer: &mut std::io::Cursor<Vec<u8>>, cache: &Cache) {
    writer.write_u8(cache.password).expect("");
    writer
        .write_u16::<LittleEndian>((cache.players).len() as u16)
        .expect("");
    writer
        .write_u16::<LittleEndian>(cache.max_players)
        .expect("");
    writer
        .write_u32::<LittleEndian>(cache.hostname.len() as u32)
        .expect("");
    writer.write(cache.hostname.as_bytes()).expect("");
    writer
        .write_u32::<LittleEndian>(cache.gamemode.len() as u32)
        .expect("");
    writer.write(cache.gamemode.as_bytes()).expect("");
    writer
        .write_u32::<LittleEndian>(cache.language.len() as u32)
        .expect("");
    writer.write(cache.language.as_bytes()).expect("");
}

pub fn send_rules(writer: &mut std::io::Cursor<Vec<u8>>, cache: &Cache) {
    writer
        .write_u16::<LittleEndian>(cache.rules.len() as u16)
        .expect("");

    for rule in cache.rules.iter() {
        writer.write_u8(rule.name.len() as u8).expect("");
        writer.write(rule.name.as_bytes()).expect("");
        writer.write_u8(rule.value.len() as u8).expect("");
        writer.write(rule.value.as_bytes()).expect("");
    }
}

pub fn send_players(writer: &mut std::io::Cursor<Vec<u8>>, cache: &Cache) {
    writer
        .write_u16::<LittleEndian>(cache.players.len() as u16)
        .expect("");

    for player in cache.players.iter() {
        writer.write_u8(player.name.len() as u8).expect("");
        writer.write(player.name.as_bytes()).expect("");
        writer.write_u8(player.score as u8).expect("");
        writer.write_u32::<LittleEndian>(player.score).expect("");
    }
}

pub fn send_detailed_players(writer: &mut std::io::Cursor<Vec<u8>>, cache: &Cache) {
    writer
        .write_u16::<LittleEndian>(cache.players.len() as u16)
        .expect("");
    for player in cache.players.iter() {
        writer.write_u8(player.name.len() as u8).expect("");
        writer.write(player.name.as_bytes()).expect("");
        writer.write_u8(player.score as u8).expect("");
        writer.write_u32::<LittleEndian>(player.score).expect("");
        writer.write_u32::<LittleEndian>(player.ping).expect("");
    }
}

pub fn send_ping(writer: &mut std::io::Cursor<Vec<u8>>, packet: QueryPacket) {
    writer.write_u8(packet.data[0]).expect("");
    writer.write_u8(packet.data[1]).expect("");
    writer.write_u8(packet.data[2]).expect("");
    writer.write_u8(packet.data[4]).expect("");
}
