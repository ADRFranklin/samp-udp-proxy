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

use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub use crate::client::Client;
pub use crate::config::{Backend, Config, Frontend};
pub use crate::query::*;
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
        return ProxyClient { config };
    }
}

pub fn bind(addr: &str) -> UdpSocket {
    match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => panic!("Failed to bind socket to {} with error {}", addr, e),
    }
}

pub fn run(proxy: Proxy) {
    let local_addr = format!("{}:{}", proxy.client.config.ip, proxy.client.config.port);
    let local = self::bind(&local_addr);
    let remote_addr = format!("{}:{}", proxy.server.config.ip, proxy.server.config.port);
    let server = Server::new(proxy.server.config);
    let cache = server.cache.clone();

    let responder = local.try_clone().expect(&format!(
        "Failed to clone local socket binding {}",
        local_addr
    ));

    let (main_sender, main_receiver) = channel::<(_, Vec<u8>)>();
    thread::spawn(move || loop {
        let (dest, buf) = main_receiver.recv().unwrap();
        let to_send = buf.as_slice();
        responder
            .send_to(to_send, dest)
            .expect(&format!("Failed to forward from server to client {}", dest));
    });

    let mut client_map = HashMap::new();
    let mut buf = [0; 64 * 1024];
    loop {
        let (num_bytes, src_addr) = local.recv_from(&mut buf).expect("No data recieved");

        // Query data
        let packet = parse_query_packet(&mut buf);
        if is_query_packet(packet.id) {
            let mut writer = write_query_header(packet.clone());

            let send = match packet.opcode as char {
                'i' => {
                    send_information(&mut writer, &cache);
                    true
                }
                'r' => {
                    send_rules(&mut writer, &cache);
                    true
                }
                'c' => {
                    send_players(&mut writer, &cache);
                    true
                }
                'd' => {
                    send_detailed_players(&mut writer, &cache);
                    true
                }
                'p' => {
                    send_ping(&mut writer, packet);
                    true
                }
                _ => false,
            };

            if send == true {
                local
                    .send_to(&writer.into_inner(), &src_addr)
                    .expect("Failed to send query data");
            }
            continue;
        }

        let mut remove_existing = false;
        loop {
            let mut ignore_failure = true;
            let client_id = format!("{}", src_addr);

            if remove_existing {
                client_map.remove(&client_id);
            }

            let sender = client_map.entry(client_id.clone()).or_insert_with(|| {
                ignore_failure = false;

                let local_send_queue = main_sender.clone();
                let (sender, reciever) = channel::<Vec<u8>>();
                let remote_addr_copy = remote_addr.clone();

                thread::spawn(move || {
                    let temp_outgoing_addr = format!("0.0.0.0:0");
                    let upstream_send = UdpSocket::bind(&temp_outgoing_addr).expect(&format!(
                        "Failed to bind to transient address {}",
                        &temp_outgoing_addr
                    ));
                    let upstream_recv = upstream_send
                        .try_clone()
                        .expect("Failed to clone client connection");

                    let mut timeouts: u64 = 0;
                    let timed_out = Arc::new(AtomicBool::new(false));

                    let local_timed_out = timed_out.clone();
                    thread::spawn(move || {
                        let mut from_upstream = [0; 64 * 1024];
                        upstream_recv
                            .set_read_timeout(Some(Duration::from_millis((3 * 60 * 100) + 100)))
                            .unwrap();
                        loop {
                            match upstream_recv.recv_from(&mut from_upstream) {
                                Ok((bytes_rcvd, _)) => {
                                    let to_send = from_upstream[..bytes_rcvd].to_vec();
                                    local_send_queue
                                        .send((src_addr, to_send))
                                        .expect("Failed to queue response from upstream");
                                }
                                Err(_) => {
                                    if local_timed_out.load(Ordering::Relaxed) {
                                        break;
                                    }
                                }
                            }
                        }
                    });

                    // Sending data from client to server
                    loop {
                        match reciever.recv_timeout(Duration::from_millis(3 * 60 * 100)) {
                            Ok(from_client) => {
                                upstream_send
                                    .send_to(from_client.as_slice(), &remote_addr_copy)
                                    .expect(&format!(
                                        "Failed to forward packet from client {} to server!",
                                        src_addr
                                    ));
                                timeouts = 0;
                            }
                            Err(_) => {
                                timeouts += 1;
                                if timeouts >= 10 {
                                    timed_out.store(true, Ordering::Relaxed);
                                    break;
                                }
                            }
                        }
                    }
                });
                sender
            });

            let to_send = buf[..num_bytes].to_vec();
            match sender.send(to_send) {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    if !ignore_failure {
                        panic!(
                            "Failed to send message to datagram forwarder for client {}",
                            client_id
                        );
                    }
                    remove_existing = true;
                    continue;
                }
            }
        }
    }
}
