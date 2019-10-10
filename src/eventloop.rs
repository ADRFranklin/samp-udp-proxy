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

use crate::packet::Packet;
use crate::proxy::Proxy;

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::io::Cursor;
use std::sync::mpsc::Receiver;
use std::{thread, time};

pub fn run(proxy: Proxy, receiver: Receiver<Packet>) {
    // Receiver
    thread::spawn(move || loop {
        let packet = receiver.recv().unwrap();

        if packet.is_query() {
            let mut writer = Cursor::new(vec![]);
            writer.write_u32::<BigEndian>(packet.packet_id).unwrap();
            writer.write_u32::<BigEndian>(packet.ip).unwrap();
            writer.write_u16::<LittleEndian>(packet.port).unwrap();
            writer.write_u8(packet.opcode).unwrap();

            let send = match packet.opcode as char {
                'i' => {
                    // information
                    true
                }
                'r' => {
                    // rules
                    true
                }
                'c' => {
                    // players
                    true
                }
                'd' => {
                    // detailed players
                    true
                }
                'p' => {
                    writer.write_u8(packet.data[0]).unwrap();
                    writer.write_u8(packet.data[1]).unwrap();
                    writer.write_u8(packet.data[2]).unwrap();
                    writer.write_u8(packet.data[4]).unwrap();
                    true
                }
                _ => false,
            };

            if send {
                let socket = &proxy.socket;
                // socket
                //     .send_to(
                //         &writer.into_inner(),
                //         String::from(packet.ip),
                //     )
                //     .unwrap();
            }
        }
    });

    loop {
        thread::sleep(time::Duration::from_millis(5));
    }
}
