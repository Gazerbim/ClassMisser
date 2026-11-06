use std::{collections::HashMap, usize::MAX};

use mio::Token;
use serde_json::Value;

use crate::services::{
    chat::Chat,
    clients::Clients,
    game::Game,
    lobby::{Lobby, PlayerState},
};

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
                _l.add_player(token);
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

    // pub fn get_players_from_lobby(&self, id: usize) -> Vec<Token> {
    //     self.list.get(&id).unwrap().get_players()
    // }

    pub fn get_spectators_from_lobby(&self, id: usize) -> Vec<Token> {
        self.list.get(&id).unwrap().get_spectators()
    }

    pub fn get_lobby_id_by_name(&self, name: &String) -> Option<usize> {
        for k in self.list.keys() {
            if self.list.get(k).unwrap().name == *name {
                return Some(k.clone());
            }
        }
        None
    }

    pub fn player_disconnection(
        &mut self,
        player_token: &Token,
        clients: &Clients,
    ) -> Vec<(usize, String)> {
        //on déconnecte les joueur de tous les lobbies dans lesquel il pourrait être
        // à priori, un joueur n'est que dans un lobby
        // mais
        // 1) les erreurs arrivent
        // 2) Le profs aimerais qu'on puisse jouer plusieurs partie à la fois. Je ne sais pas si on le fera, mais c'est déjà ca de fait
        // (c'est pas du tout psk j'ai oublié pourquoi j'ai fait ça il y a deux semaines)
        let affected_lobbies: Vec<usize> = self
            .list
            .keys()
            .filter(|k| self.list.get(k).unwrap().is_player_in_game(*player_token))
            .cloned()
            .collect();
        let mut empty_lobbies: Vec<String> = Vec::new();
        let mut return_messages: Vec<(usize, String)> = Vec::new();
        for k in affected_lobbies {
            let lobby = self.list.get_mut(&k).unwrap();
            lobby.remove_player(player_token);
            if lobby.is_lobby_empty() {
                empty_lobbies.push(lobby.name.clone());
            } else {
                let mut new_players: Vec<String> = Vec::new();
                let mut spectators: Vec<String> = Vec::new();
                for t in lobby.get_players() {
                    let username = match clients.get_username(&t) {
                        Some(_u) => _u,
                        None => "disconnected".to_string(),
                    };
                    if username != "".to_string() {
                        new_players.push(username.clone());
                    }
                }
                for t in lobby.get_spectators() {
                    let username = match clients.get_username(&t) {
                        Some(_u) => _u,
                        None => "disconnected".to_string(),
                    };
                    if username != "".to_string() {
                        spectators.push(username.clone());
                    }
                }
                let message = serde_json::json!({
                    "type": "game",
                    "header": "update_players",
                    "data": {
                        "new_players": new_players,
                        "new_spectators": spectators
                    }
                })
                .to_string()
                    + "$";
                return_messages.push((k, message));
                if lobby.owner == *player_token {
                    lobby.owner = lobby.list_players[0];
                }
            }
        }
        for n in empty_lobbies {
            self.delete_lobby_by_name(n);
        }

        return return_messages;
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
        clients: &Clients,
    ) -> (String, String) {
        let mut broadcast = "".to_string();
        let mut type_response = "lobbies";
        let mut response_header = "";
        let response_data: Value = match header {
            "list_available_lobbies" => {
                let mut lobby_list: Vec<String> = Vec::new();
                for k in self.list.keys() {
                    match self.list.get(k) {
                        Some(l) => lobby_list.push(l.name.clone()),
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
                    .get_lobby_id_by_name(&data["name"].to_string())
                    .is_some()
                {
                    response_header = "error";
                    return (
                        "".to_string(),
                        serde_json::json!({
                            "type": "lobbies",
                            "header": response_header,
                            "data": serde_json::json!({"message": "Lobby name already taken"}),
                        })
                        .to_string()
                            + "$",
                    );
                }
                let mut new_id = self.list.len();
                if self.availables_ids_for_lobby.len() > 0 {
                    new_id = self.availables_ids_for_lobby[0];
                    self.availables_ids_for_lobby.remove(0);
                }
                self.list.insert(
                    new_id,
                    Lobby {
                        name: data["name"].to_string().clone()
                            [1..(data["name"].to_string().len() - 1)]
                            .to_string(),
                        list_players: vec![sender],
                        in_game: false,
                        owner: sender,
                        game: Game::empty(),
                        chat: Chat::new(),
                        number_of_players: 0,
                        player_states: Vec::new(),
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
                    if l.name
                        == data["name"].to_string().clone()[1..(data["name"].to_string().len() - 1)]
                            .to_string()
                    {
                        let mut already_in = false;
                        for p in l.list_players.iter() {
                            if *p == sender {
                                already_in = true;
                                break;
                            }
                        }
                        if !already_in {
                            //l.list_players.push(sender);
                            l.add_player(sender);
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
                    let name = data["name"].to_string();
                    let name = name.clone()[1..name.len() - 1].to_string();
                    if l.name == name {
                        l.remove_player(&sender);
                        broadcast = l.name.clone();
                        let mut players: Vec<String> = Vec::new();
                        let mut spectators: Vec<String> = Vec::new();
                        for t in l.get_players() {
                            let username = match clients.get_username(&t) {
                                Some(_u) => _u,
                                None => "disconnected".to_string(),
                            };
                            players.push(username);
                        }
                        for t in l.get_spectators() {
                            let username = match clients.get_username(&t) {
                                Some(_u) => _u,
                                None => "disconnected".to_string(),
                            };
                            spectators.push(username);
                        }
                        data_value = serde_json::json!({ "new_players": players, "new_spectators": spectators});
                        response_header = "update_players";
                        type_response = "game";
                        //à revoir ! c'est toujours faux!!!!
                        if l.is_lobby_empty() {
                            lobby_name_to_delete = l.name.clone();
                        }
                        break;
                    }
                }
                self.delete_lobby_by_name(lobby_name_to_delete);
                data_value
            }
            _ => serde_json::json!({}),
        };
        if response_data == serde_json::json!({}) {
            return ("".to_string(), String::from(""));
        }
        (
            broadcast,
            serde_json::json!({
                "type": type_response,
                "header": response_header,
                "data": response_data,
            })
            .to_string()
                + "$",
        )
    }
}
