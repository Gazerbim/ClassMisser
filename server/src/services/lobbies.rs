use mio::Token;
use serde_json::Value;

use crate::services::players;
pub struct Lobby {
    pub name: String,
    pub list_players: Vec<Token>,
    pub in_game: bool,
    pub owner: Token,
}

pub struct Lobbies {
    pub list: Vec<Lobby>,
}

impl Lobbies {
    pub fn new() -> Lobbies {
        Lobbies { list: Vec::new() }
    }

    pub fn add_player(&mut self, token: Token, lobby: usize) {
        self.list[lobby].list_players.push(token);
    }

    pub fn handle_request(&mut self, sender: Token, header: &str, data: &Value) -> String {
        let response_data: Value = match (header) {
            "list_available_lobbies" => {
                let mut lobby_list: Vec<String> = Vec::new();
                for l in &self.list {
                    lobby_list.push(l.name.clone()[1..(l.name.len() - 1)].to_string());
                }
                serde_json::json!({
                    "lobbies": lobby_list,
                })
            }
            "create_lobby" => {
                self.list.push(Lobby {
                    name: data["name"].to_string(),
                    list_players: vec![sender],
                    in_game: false,
                    owner: sender,
                });
                serde_json::json!({
                    "accept": true,
                })
            }
            "enter_lobby" => {
                let mut added = false;
                for l in self.list.iter_mut() {
                    if l.name == data["name"].to_string() {
                        let mut already_in = false;
                        for p in l.list_players.iter_mut() {
                            if *p == sender {
                                already_in = true;
                                break;
                            }
                        }
                        if !already_in {
                            l.list_players.push(sender);
                            added = true;
                        }
                    }
                }
                serde_json::json!({"accept": added})
            }
            _ => serde_json::json!({}),
        };
        serde_json::json!({
            "type": header,
            "data": response_data,
        })
        .to_string()
            + "\n"
    }
}
