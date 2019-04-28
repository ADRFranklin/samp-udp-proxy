/*
     ___  __ _ _ __ ___  _ __    _ __  _ __ _____  ___   _
    / __|/ _` | '_ ` _ \| '_ \  | '_ \| '__/ _ \ \/ / | | |
    \__ \ (_| | | | | | | |_) | | |_) | | | (_) >  <| |_| |
    |___/\__,_|_| |_| |_| .__/  | .__/|_|  \___/_/\_\\__, |
                        |_|     |_|                  |___/

    @Author         Arron (Michael) Franklin
    @File           main.rs
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
//  Crates
// --

extern crate byteorder; // 1.3.1
extern crate rand;

// --
//  Libs
// --

pub use samp_udp_proxy::config::Config;
pub use samp_udp_proxy::proxy::Proxy;
pub use samp_udp_proxy::server::Server;

// --
//  Main
// --

const CONFIG: &str = "config.toml";

pub fn main() {
    let config = Config::parse(CONFIG.into()).unwrap();
    Proxy::new(config.frontend).start(Server::new(config.backend));
}
