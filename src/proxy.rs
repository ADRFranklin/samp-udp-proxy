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

use std::io::Cursor;
use std::net::UdpSocket;
use std::thread;

pub use crate::config::Frontend;
pub use crate::packet::Packet;
pub use crate::query::Query;
pub use crate::server::Server;

// --
//  Structs
// --

#[derive(Debug)]
pub struct Proxy {
    pub ip: String,
    pub port: u16,
}
// --
//  API
// --

impl Proxy {
    pub fn new(config: Frontend) -> Proxy {
        return Proxy {
            ip: config.ip,
            port: config.port,
        };
    }

    pub fn start(&self, server: Server) {
        let socket = UdpSocket::bind(format!("{}:{}", self.ip, self.port))
            .expect("couldn't bind to address");

        let handle = thread::spawn(move || {
            let mut buf = [0; 32];
            loop {
                let src = match socket.recv_from(&mut buf) {
                    Ok((_, src)) => src,
                    Err(e) => {
                        println!("couldn't recieve a datagram: {}", e);
                        continue;
                    }
                };

                let data: &[u8] = &buf;
                let packet = Packet::new(&mut Cursor::new(data));

                if packet.is_query_packet() {
                    let mut writer = Cursor::new(vec![]);
                    let query = Query::new(packet.clone());

                    match packet.data[0] as char {
                        'i' => {
                            query.get_info(&server, &mut writer);

                            match socket.send_to(&writer.into_inner(), src) {
                                Ok(_) => (),
                                Err(e) => println!("Failed to send packet: {}", e),
                            }
                        }

                        'r' => {
                            query.get_rules(&server, &mut writer);

                            let result = writer.into_inner();
                            match socket.send_to(&result, src) {
                                Ok(_) => (),
                                Err(e) => println!("Failed to send packet: {}", e),
                            }
                        }

                        'c' => {
                            query.get_players(&server, &mut writer);

                            let result = writer.into_inner();
                            match socket.send_to(&result, src) {
                                Ok(_) => (),
                                Err(e) => println!("Failed to send packet: {}", e),
                            }
                        }

                        'd' => {
                            query.get_player_details(&server, &mut writer);

                            let result = writer.into_inner();
                            match socket.send_to(&result, src) {
                                Ok(_) => (),
                                Err(e) => println!("Failed to send packet: {}", e),
                            }
                        }

                        'p' => {
                            query.get_ping(&mut writer);

                            let result = writer.into_inner();
                            match socket.send_to(&result, src) {
                                Ok(_) => (),
                                Err(e) => println!("Failed to send packet: {}", e),
                            }
                        }

                        _ => (),
                    };
                }

                //println!("{:?}", packet);
            }
        });

        handle.join().unwrap();
    }
}
