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

use std::io::Write;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

pub use crate::packet::Packet;
pub use crate::server::{Server, Cache, Player, Rule};

// --
//  SAMP Query
// --

pub struct Query {
    id: u32,
    ip: u32,
    port: u16,
    opcode: u8,
    packet: Packet,
}

impl Query {
    pub fn new(packet: Packet) -> Query {        
        return Query {
            id: packet.id,
            ip: packet.ip,
            port: packet.port,
            opcode: packet.data[0],
            packet: packet,
        }
    }

    pub fn get_info(&self, server: &Server, writer: &mut std::io::Cursor<Vec<u8>>) {
        writer.write_u32::<BigEndian>(self.id).expect("");
        writer.write_u32::<BigEndian>(self.ip).expect("");
        writer.write_u16::<BigEndian>(self.port).expect("");
        writer.write_u8(self.opcode).expect("");

        writer.write_u8(server.cache.password).expect("");
        writer.write_u16::<LittleEndian>((server.cache.players).len() as u16).expect("");
        writer.write_u16::<LittleEndian>(server.cache.max_players).expect("");
        writer.write_u32::<LittleEndian>(server.cache.hostname.len() as u32).expect("");
        writer.write(server.cache.hostname.as_bytes()).expect("");
        writer.write_u32::<LittleEndian>(server.cache.gamemode.len() as u32).expect("");
        writer.write(server.cache.gamemode.as_bytes()).expect("");
        writer.write_u32::<LittleEndian>(server.cache.language.len() as u32).expect("");
        writer.write(server.cache.language.as_bytes()).expect("");
    }

    pub fn get_rules(&self, server: &Server, writer: &mut std::io::Cursor<Vec<u8>>){
        writer.write_u32::<BigEndian>(self.id).expect("");
        writer.write_u32::<BigEndian>(self.ip).expect("");
        writer.write_u16::<BigEndian>(self.port).expect("");
        writer.write_u8(self.opcode).expect("");

        writer.write_u16::<LittleEndian>(server.cache.rules.len() as u16).expect("");
        for rule in server.cache.rules.iter() {
            writer.write_u8(rule.name.len() as u8).expect("");
            writer.write(rule.name.as_bytes()).expect("");
            writer.write_u8(rule.value.len() as u8).expect("");
            writer.write(rule.value.as_bytes()).expect("");
        }
    }

    pub fn get_players(&self, server: &Server, writer: &mut std::io::Cursor<Vec<u8>>) {
        writer.write_u32::<BigEndian>(self.id).expect("");
        writer.write_u32::<BigEndian>(self.ip).expect("");
        writer.write_u16::<BigEndian>(self.port).expect("");
        writer.write_u8(self.opcode).expect("");

        writer.write_u16::<LittleEndian>(server.cache.players.len() as u16).expect("");
        for player in server.cache.players.iter() {
            writer.write_u8(player.name.len() as u8).expect("");
            writer.write(player.name.as_bytes()).expect("");
            writer.write_u8(player.score as u8).expect("");
            writer.write_u32::<LittleEndian>(player.score).expect("");
        }     
    }

    pub fn get_player_details(&self, server: &Server, writer: &mut std::io::Cursor<Vec<u8>>) {
        writer.write_u32::<BigEndian>(self.id).expect("");
        writer.write_u32::<BigEndian>(self.ip).expect("");
        writer.write_u16::<BigEndian>(self.port).expect("");
        writer.write_u8(self.opcode).expect("");

        writer.write_u16::<LittleEndian>(server.cache.players.len() as u16).expect("");
        for player in server.cache.players.iter() {
            writer.write_u8(player.name.len() as u8).expect("");
            writer.write(player.name.as_bytes()).expect("");
            writer.write_u8(player.score as u8).expect("");
            writer.write_u32::<LittleEndian>(player.score).expect("");
            writer.write_u32::<LittleEndian>(player.ping).expect("");             
        }    
    }

    pub fn get_ping(&self, writer: &mut std::io::Cursor<Vec<u8>>) {
        writer.write_u32::<BigEndian>(self.id).expect("");
        writer.write_u32::<BigEndian>(self.ip).expect("");
        writer.write_u16::<BigEndian>(self.port).expect("");
        writer.write_u8(self.opcode).expect("");

        writer.write_u8(self.packet.data[1]).expect("");
        writer.write_u8(self.packet.data[2]).expect("");
        writer.write_u8(self.packet.data[3]).expect("");
        writer.write_u8(self.packet.data[4]).expect("");
    }
}