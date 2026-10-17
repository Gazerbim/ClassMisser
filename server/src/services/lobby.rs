use mio::Token;
use serde_json::Value;

use crate::services::game::{self, Game};

pub struct Lobby {
    pub name: String,
    pub list_players: Vec<Token>,
    pub in_game: bool,
    pub owner: Token,
    pub game: Game,
    pub number_of_players: i8,
}

impl Lobby {
    pub fn start_game(
        &mut self,
        number_team: i8,
        board_size: usize,
    ) -> (Vec<usize>, Vec<usize>, Vec<i8>) {
        self.game = Game::new(number_team, board_size);
        self.number_of_players = number_team;
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
        for i in 0..self.number_of_players {
            players.push(self.list_players[i as usize]);
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

    pub fn remove_player(&mut self, player_token: &Token) {
        for i in 0..self.list_players.len() {
            if self.list_players[i] == *player_token {
                self.list_players.remove(i);
                break;
            }
        }
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
                _ => (Token(0), String::from("")),
            },
            None => (Token(0), String::from("")),
        }
    }
}
