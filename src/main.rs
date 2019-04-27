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
//  Libraries (What libraries?)
// --

#[macro_use]

// --
//  Modules (the modules)
// --

mod config;
mod servers;

// --
//  Main (the main function)
// --

pub fn main() {
    let path = "config.toml".to_string();

    // Load Config
    let config = config::parse(path);

    // Add config servers as a server instance
    for cserver in config.servers {
        if cserver.name && cserver.ip && cserver.port {
            servers::Servers::new(cserver.name, cserver.ip, cserver.port);
        }
    }
}
