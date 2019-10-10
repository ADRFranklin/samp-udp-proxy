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

pub struct Cache {
    pub hostname: String,
    pub current_players: i32,
    pub max_players: i32,
    pub mode: String,
    pub language: String,
    pub passworded: bool,
    pub rules: Vec<Rule>,
    pub players: Vec<Player>,
}

pub struct Rule {
    pub name: String,
    pub value: String,
}

pub struct Player {
    pub name: String,
    pub score: i32,
}

impl Default for Cache {
    fn default() -> Cache {
        Cache {
            hostname: "Proxy Server".to_string(),
            current_players: 0,
            max_players: 0,
            mode: "v1.2.3".to_string(),
            language: "English".to_string(),
            passworded: false,
            rules: vec![],
            players: vec![],
        }
    }
}
