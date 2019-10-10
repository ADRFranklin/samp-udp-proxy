/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           client.rs
    @Project        SA-MP Proxy
    @Created        20th March 2019
    @Weburl         https://sanandreasgaming.com
                    https://burgershot.gg/member.php?action=profile&uid=5

    - README -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/README.md

    - LICENCE -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/LICENSE
*/

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::prelude::*;

#[derive(Debug)]
pub struct Packet {
    pub packet_id: u32,
    pub ip: u32,
    pub port: u16,
    pub opcode: u8,
    pub data: Vec<u8>,
}

impl Packet {
    // Create a new instance of Packet
    pub fn new<R: BufRead + Seek>(reader: &mut R) -> Packet {
        let packet_id = reader.read_u32::<BigEndian>().unwrap();
        let ip = reader.read_u32::<BigEndian>().unwrap();
        let port = reader.read_u16::<LittleEndian>().unwrap();
        let opcode = reader.read_u8().unwrap();
        let mut data = Vec::new();
        let _ = reader.read_to_end(&mut data);

        Packet {
            packet_id,
            ip,
            port,
            opcode,
            data,
        }
    }

    // Is the packet of type query
    pub fn is_query(&self) -> bool {
        self.packet_id == 1_396_788_560
    }
}
