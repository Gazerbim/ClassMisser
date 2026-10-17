use std::collections::HashMap;

use mio::Token;
use serde_json::Value;

use crate::services::{game::Game, lobby::Lobby};

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

    pub fn get_clients_from_lobby(&self, id: usize) -> Vec<Token> {
        self.list.get(&id).unwrap().get_all_players()
    }

    pub fn get_players_from_lobby(&self, id: usize) -> Vec<Token> {
        self.list.get(&id).unwrap().get_players()
    }

    pub fn get_spectators_from_lobby(&self, id: usize) -> Vec<Token> {
        self.list.get(&id).unwrap().get_spectators()
    }

    pub fn get_lobby_id_by_name(&mut self, name: String) -> Option<usize> {
        for k in self.list.keys() {
            if self.list.get(k).unwrap().name == name {
                return Some(k.clone());
            }
        }
        None
    }

    pub fn player_disconnection(&mut self, player_token: &Token) {
        let affected_lobbies: Vec<usize> = self
            .list
            .keys()
            .filter(|k| self.list.get(k).unwrap().is_player_in_game(*player_token))
            .cloned()
            .collect();
        let mut empty_lobbies: Vec<String> = Vec::new();
        for k in affected_lobbies {
            let lobby = self.list.get_mut(&k).unwrap();
            lobby.remove_player(player_token);
            if lobby.get_number_players_in_lobby() == 0 {
                empty_lobbies.push(lobby.name.clone());
            } else if lobby.owner == *player_token {
                lobby.owner = lobby.list_players[0];
            }
        }
        for n in empty_lobbies {
            self.delete_lobby_by_name(n);
        }
    }

    pub fn start_game_by_lobby_id(
        &mut self,
        id: usize,
        number_team: i8,
        board_size: usize,
    ) -> (Vec<usize>, Vec<usize>, Vec<i8>) {
        self.list
            .get_mut(&id)
            .unwrap()
            .start_game(number_team, board_size)
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
                //check if the name is already taken
                if self
                    .get_lobby_id_by_name(data["name"].to_string())
                    .is_some()
                {
                    response_header = "error";
                    return serde_json::json!({
                        "type": "lobbies",
                        "header": response_header,
                        "data": serde_json::json!({"message": "Lobby name already taken"}),
                    })
                    .to_string()
                        + "$";
                }
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
                        game: Game::empty(),
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
            + "$"
    }
}
