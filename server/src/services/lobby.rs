use std::usize::MAX;

use mio::Token;
use serde_json::Value;

use crate::services::{chat::Chat, clients::Clients, game::Game};

#[derive(PartialEq)]
pub enum PlayerState {
    Playing,
    Lost,
    Disconnected,
}

pub struct Lobby {
    pub name: String,
    pub list_players: Vec<Token>,
    pub in_game: bool,
    pub owner: Token,
    pub game: Game,
    pub number_of_players: i8, //nombre de joueurs, pas nombre de connectés dans le lobby
    pub player_states: Vec<PlayerState>,
    pub chat: Chat, //miaou
}

impl Lobby {
    pub fn start_game(
        &mut self,
        number_team: i8,
        board_size: usize,
    ) -> (Vec<usize>, Vec<usize>, Vec<i8>) {
        self.game = Game::new(number_team, board_size);
        self.game
            .pieces
            .set_king_index(self.game.pieces.get_first_king_index());
        self.number_of_players = number_team;
        for i in 0..number_team {
            self.player_states.push(PlayerState::Playing);
        }
        self.in_game = true;
        self.game.board.get_simplfied_board()
    }

    pub fn get_number_players_in_lobby(&self) -> usize {
        self.list_players.len()
    }

    pub fn get_all_players(&self) -> Vec<Token> {
        let mut players: Vec<Token> = Vec::new();
        for i in 0..self.list_players.len() {
            players.push(self.list_players[i as usize]);
        }
        players
    }

    pub fn get_players(&self) -> Vec<Token> {
        let mut players: Vec<Token> = Vec::new();
        // for i in 0..self.list_players.len() {
        //     println!("{}", self.list_players[i].0);
        // }
        for i in 0..self.number_of_players {
            if i < self.list_players.len() as i8 {
                players.push(self.list_players[i as usize]);
            }
        }
        players
    }

    pub fn get_spectators(&self) -> Vec<Token> {
        let mut spectators: Vec<Token> = Vec::new();
        for i in (self.number_of_players as usize)..self.list_players.len() {
            spectators.push(self.list_players[i]);
        }
        spectators
    }

    pub fn is_player_in_game(&self, target: Token) -> bool {
        for p in self.list_players.iter() {
            if *p == target {
                return true;
            }
        }
        false
    }

    pub fn add_player(&mut self, player_token: Token) -> usize {
        //modifier pour quand un joueur se reconnecte
        if self.list_players.len() <= self.number_of_players as usize {
            //pas assez de joueur se sont connectés
            // ou des joueurs se sont deconnectés
            let mut losts = 0;
            //pas encore ou plus assez de joueurs
            for i in 0..self.player_states.len() {
                if self.player_states[i] == PlayerState::Disconnected {
                    //un joueur s'est déconnecté, le nouveau joueur va le remplacer
                    if self.list_players.len() <= i {
                        self.list_players.push(player_token);
                    } else {
                        self.list_players[i] = player_token;
                    }
                    self.player_states[i] = PlayerState::Playing;
                    self.game.set_player_afk_state(i, false);
                    return i; //on a ajouté le joueur
                } else if self.player_states[i] == PlayerState::Lost {
                    losts += 1;
                }
            }
            if self.list_players.len() + losts == self.number_of_players as usize {
                self.list_players.push(player_token);
                return 5;
            }
            self.list_players.push(player_token);
            self.player_states.push(PlayerState::Playing);
            return self.list_players.len() - 1;
        } else {
            self.list_players.push(player_token);
            return 5;
        }
    }

    pub fn lobby_is_in_game(&self) -> bool {
        self.in_game
    }

    pub fn remove_player(&mut self, player_token: &Token) -> usize {
        let mut index_removed = MAX;
        for i in 0..self.list_players.len() {
            if self.list_players[i] == *player_token {
                if i < self.number_of_players as usize {
                    if self.number_of_players < self.list_players.len() as i8
                        || self.in_game == false
                    {
                        self.list_players[i] = self.list_players[self.number_of_players as usize];
                        self.list_players.remove(self.number_of_players as usize);
                    } else {
                        if self.player_states[i] != PlayerState::Lost {
                            self.player_states[i] = PlayerState::Disconnected;
                        }
                        self.game.set_player_afk_state(i, true);
                        if self.game.get_player_playing() == i as i8 {
                            self.game.end_turn();
                        }
                    }
                } else {
                    self.list_players.remove(i);
                }
                if self.owner == *player_token && self.list_players.len() != 0 {
                    self.owner = self.list_players[0];
                }
                index_removed = i;
                break;
            }
        }
        index_removed
    }

    pub fn is_lobby_empty(&self) -> bool {
        if self.list_players.len() > self.number_of_players as usize {
            return false;
        } else {
            let mut n = 0;
            for i in 0..self.player_states.len() {
                if !(self.player_states[i] == PlayerState::Disconnected) {
                    n += 1;
                }
            }
            if n == 0 {
                return true;
            }
        }
        false
    }

    pub fn get_lobby_info(&self, clients: &Clients) -> Value {
        let simplified_board = self.game.get_simplified_board();
        let size = self.game.get_board_size();
        let players = self.number_of_players;
        let mut player_usernames: Vec<String> = Vec::new();
        for t in self.get_players() {
            let username = clients.get_username(&t).unwrap();
            player_usernames.push(username);
        }
        let mut spectator_usernames: Vec<String> = Vec::new();
        for t in self.get_spectators() {
            let username = clients.get_username(&t).unwrap();
            spectator_usernames.push(username);
        }
        serde_json::json!({
            "in_game": true,
            "board_size": size,
            "number_player": players,
            "players": player_usernames,
            "spectators": spectator_usernames,
            "x": simplified_board.0,
            "y": simplified_board.1,
            "pieces": simplified_board.2,
        })
    }

    pub fn handle_request(
        &mut self,
        sender: Token,
        request_type: &str,
        header: &str,
        data: &Value,
    ) -> (Token, String) {
        let sender_id = self.list_players.iter().position(|t| *t == sender);
        match sender_id {
            Some(_sender) => match request_type {
                "game" => {
                    let (who, message) = self.game.handle_request(_sender as i8, header, data);
                    if message != "".to_string() {
                        if who < 0 {
                            (Token(0), message)
                        } else {
                            (self.list_players[who as usize], message)
                        }
                    } else {
                        (Token(0), String::from(""))
                    }
                }
                "chat" => {
                    let mut n_sender = self.list_players.iter().position(|&t| t == sender).unwrap();
                    if n_sender >= self.number_of_players as usize {
                        n_sender = 5;
                    }
                    (Token(0), self.chat.handle_request(n_sender, header, data))
                }
                _ => (Token(0), String::from("")),
            },
            None => (Token(0), String::from("")),
        }
    }
}
