/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           proxy.rs
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

pub use crate::client::Client;
pub use crate::config::{Backend, Config, Frontend};
pub use crate::packet::Packet;
pub use crate::query::Query;
pub use crate::server::{Cache, Server};

// --
//  Structs
// --

#[derive(Debug)]
pub struct Proxy {
    server: ProxyServer,
    client: ProxyClient,
}

#[derive(Debug)]
pub struct ProxyServer {
    pub config: Backend,
}

#[derive(Debug)]
pub struct ProxyClient {
    pub config: Frontend,
    pub clients: Vec<Client>,
}

// --
//  API
// --

impl Proxy {
    pub fn new(config: Config) -> Proxy {
        return Proxy {
            server: ProxyServer::new(config.backend),
            client: ProxyClient::new(config.frontend),
        };
    }
}

impl ProxyServer {
    fn new(config: Backend) -> ProxyServer {
        return ProxyServer { config };
    }
}

impl ProxyClient {
    fn new(config: Frontend) -> ProxyClient {
        return ProxyClient {
            config,
            clients: vec![],
        };
    }
}

pub fn bind(addr: &str) -> UdpSocket {
    match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => panic!("Failed to bind socket to {} with error {}", addr, e),
    }
}

pub fn run(proxy: Proxy) {
    let socket = self::bind(&format!(
        "{}:{}",
        proxy.client.config.ip, proxy.client.config.port
    ));
    let server = Server::new(proxy.server.config);

    // client
    let thread = thread::spawn(move || {
        let mut buf = [0; 512];
        loop {
            let src = match socket.recv_from(&mut buf) {
                Ok((_, src)) => src,
                Err(e) => {
                    println!("couldn't recieve a datagram: {}", e);
                    continue;
                }
            };

            // Client
            let data: &[u8] = &buf;
            let packet = Packet::new(&mut Cursor::new(data));

            // Is Client
            if packet.is_query_packet() {
                let query = Query::new(packet.clone(), server.cache.clone());
                let mut writer = Cursor::new(vec![]);

                let send = match packet.data[0] as char {
                    'i' => {
                        query.send_information(&mut writer);
                        true
                    }
                    'r' => {
                        query.send_rules(&mut writer);
                        true
                    }
                    'c' => {
                        query.send_players(&mut writer);
                        true
                    }
                    'd' => {
                        query.send_detailed_players(&mut writer);
                        true
                    }
                    'p' => {
                        query.send_ping(&mut writer);
                        true
                    }
                    _ => false,
                };

                if send {
                    match socket.send_to(&writer.into_inner(), src) {
                        Ok(_) => (),
                        Err(e) => println!("failed to send packet: {}", e),
                    }
                }
            }
        }
    });

    thread.join().expect("Client thread has panicked");
}
