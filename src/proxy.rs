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

use crate::packet::Packet;
use std::io::Cursor;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub struct Proxy {
    pub socket: UdpSocket,
    sender: Sender<Packet>,
    threads: u32,
}

impl Proxy {
    pub fn new(address: &str, threads: u32, sender: Sender<Packet>) -> Proxy {
        let socket = Self::bind_server(address);

        Proxy {
            socket,
            sender,
            threads,
        }
    }

    fn bind_server(address: &str) -> UdpSocket {
        match UdpSocket::bind(&address) {
            Ok(s) => {
                println!("Server Listening: {}", address);
                s
            }
            Err(e) => panic!("Bind socket failed {}: {}", address, e),
        }
    }

    pub fn spawn_threads(&self) -> Vec<JoinHandle<()>> {
        (0..self.threads)
            .map(|_| {
                let thread_socket = self.socket.try_clone().unwrap();
                let thread_sender = self.sender.clone();

                let mut buf = [0; 512];

                thread::spawn(move || loop {
                    match thread_socket.recv_from(&mut buf) {
                        Ok((_, _)) => {
                            let data: &[u8] = &buf;
                            let packet = Packet::new(&mut Cursor::new(data));
                            thread_sender.send(packet).unwrap();
                        }
                        Err(e) => {
                            println!("Error receiving: {}", e);
                        }
                    }
                })
            })
            .collect()
    }
}
