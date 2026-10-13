use std::collections::HashMap;

use mio::Token;
use serde_json::Value;

use crate::services::lobby::Lobby;

pub struct Lobbies {
    pub list: HashMap<usize, Lobby>,
    pub availables_ids_for_lobby: Vec<usize>,
}

impl Lobbies {
    pub fn new() -> Lobbies {
        Lobbies {
            list: HashMap::new(),
            availables_ids_for_lobby: Vec::new(),
        }
    }

    pub fn add_player(&mut self, token: Token, lobby: usize) -> bool {
        match self.list.get_mut(&lobby) {
            Some(_l) => {
                _l.list_players.push(token);
                true
            }
            None => false,
        }
    }

    pub fn get_player_lobby_id(&self, player: Token) -> i32 {
        for i in 0..self.list.len() {
            if self.list[&i].is_player_in_game(player) {
                return i as i32;
            }
        }
        -1
    }

    pub fn get_lobby_id_by_name(&mut self, name: String) -> Option<usize> {
        for k in self.list.keys() {
            if self.list.get(k).unwrap().name == name {
                return Some(k.clone());
            }
        }
        None
    }

    pub fn delete_lobby_by_name(&mut self, name: String) {
        let to_remove: Vec<usize> = self
            .list
            .keys()
            .filter(|k| self.list.get(k).unwrap().name == name)
            .cloned()
            .collect();
        self.availables_ids_for_lobby.extend(&to_remove);
        for k in to_remove {
            self.list.remove(&k);
        }
    }

    pub fn handle_request(
        &mut self,
        sender: Token,
        header: &str,
        data: &Value,
        receiver: &mut Token,
    ) -> String {
        let mut response_header = "";
        let response_data: Value = match header {
            "list_available_lobbies" => {
                let mut lobby_list: Vec<String> = Vec::new();
                for k in self.list.keys() {
                    match self.list.get(k) {
                        Some(l) => {
                            lobby_list.push(l.name.clone()[1..(l.name.len() - 1)].to_string())
                        }
                        None => {}
                    }
                }
                response_header = "list_available_lobbies";
                serde_json::json!({
                    "lobbies": lobby_list,
                })
            }

            "create_lobby" => {
                let mut new_id = self.list.len();
                if self.availables_ids_for_lobby.len() > 0 {
                    new_id = self.availables_ids_for_lobby[0];
                    self.availables_ids_for_lobby.remove(0);
                }
                self.list.insert(
                    new_id,
                    Lobby {
                        name: data["name"].to_string(),
                        list_players: vec![sender],
                        in_game: false,
                        owner: sender,
                        game: None,
                        number_of_players: 0,
                    },
                );
                response_header = "created_lobby";

                let name = data["name"].to_string();
                serde_json::json!({
                    "name": name.clone()[1..(name.len() - 1)].to_string()
                })
            }
            "enter_lobby" => {
                let mut added = String::from("");
                for l in self.list.values_mut() {
                    if l.name == data["name"].to_string() {
                        let mut already_in = false;
                        for p in l.list_players.iter() {
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
                let mut data_value: Value = serde_json::json!({});
                let mut lobby_name_to_delete = "".to_string();
                for l in self.list.values_mut() {
                    if l.name == data["name"].to_string() {
                        let before_size = l.list_players.len();
                        for i in 0..l.list_players.len() {
                            if l.list_players[i] == sender {
                                l.list_players.remove(i);
                                if sender == l.owner && before_size > 1 {
                                    l.owner = l.list_players[0];
                                    data_value = serde_json::json!({"name": l.name.clone()});
                                    response_header = "grant_ownership";
                                    *receiver = l.owner.clone();
                                }
                                break;
                            }
                        }
                        if before_size != l.list_players.len() {
                            if l.list_players.len() == 0 {
                                lobby_name_to_delete = l.name.clone();
                            }
                            break;
                        }
                    }
                }
                self.delete_lobby_by_name(lobby_name_to_delete);
                data_value
            }
            _ => serde_json::json!({}),
        };
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
