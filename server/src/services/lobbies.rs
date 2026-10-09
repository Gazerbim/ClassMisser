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
        let mut response_header = "";
        let response_data: Value = match header {
            "list_available_lobbies" => {
                let mut lobby_list: Vec<String> = Vec::new();
                for l in &self.list {
                    lobby_list.push(l.name.clone()[1..(l.name.len() - 1)].to_string());
                }
                response_header = "list_available_lobbies";
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
                response_header = "created_lobby";

                let name = data["name"].to_string();
                serde_json::json!({
                    "name": name.clone()[1..(name.len() - 1)].to_string()
                })
            }
            "enter_lobby" => {
                let mut added = String::from("");
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
                            let name = data["name"].to_string();
                            added = name.clone()[1..(name.len() - 1)].to_string()
                        }
                    }
                }
                response_header = "enter_lobby";
                serde_json::json!({"name": added})
            }
            "quit_lobby" => {
                let mut index = 0;
                for l in self.list.iter_mut() {
                    if l.name == data["name"].to_string() {
                        let before_size = l.list_players.len();
                        for i in 0..l.list_players.len() {
                            if l.list_players[i] == sender {
                                l.list_players.remove(i);
                                break;
                            }
                        }
                        if before_size != l.list_players.len() {
                            if l.list_players.len() == 0 {
                                self.list.remove(index);
                            }
                            break;
                        }
                    }
                    index += 1;
                }
                serde_json::json!({})
            }
            _ => serde_json::json!({}),
        };
        for l in self.list.iter_mut() {
            println!("Lobby name: {}, size: {}", l.name, l.list_players.len());
        }
        if response_data == serde_json::json!({}) {
            return String::from("");
        }
        serde_json::json!({
            "type": "lobbies",
            "header": response_header,
            "data": response_data,
        })
        .to_string()
            + "\n"
    }
}
