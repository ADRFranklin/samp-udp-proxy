/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           packet.rs
    @Project        SA-MP Proxy
    @Created        20th March 2019
    @Weburl         https://sanandreasgaming.com
                    https://burgershot.gg/member.php?action=profile&uid=5

    - README -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/README.md

    - LICENSE -
    https://github.com/ADRFranklin/samp-udp-proxy/blob/master/LICENSE
*/

use byteorder::{BigEndian, ReadBytesExt};
use std::io::prelude::*;
use std::vec::Vec;

// --
//  Structs
// --

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u32,
    pub ip: u32,
    pub port: u16,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new<R: BufRead + Seek>(reader: &mut R) -> Packet {
        let id = reader.read_u32::<BigEndian>().unwrap();
        let ip = reader.read_u32::<BigEndian>().unwrap();
        let port = reader.read_u16::<BigEndian>().unwrap();
        let mut data = Vec::new();
        let _ = reader.read_to_end(&mut data);

        Packet { id, ip, port, data }
    }

    pub fn is_query_packet(&self) -> bool {
        if self.id == 1396788560 {
            return true;
        }
        return false;
    }
}

