use mio::Token;
use serde_json::Value;

use crate::services::game::Game;

pub struct Lobby {
    pub name: String,
    pub list_players: Vec<Token>,
    pub in_game: bool,
    pub owner: Token,
    pub game: Option<Game>,
    pub number_of_players: usize,
}

impl Lobby {
    pub fn start_game(&mut self) {
        self.game = Some(Game::new());
    }

    pub fn get_number_players_in_lobby(&self) -> usize {
        self.list_players.len()
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

    pub fn handle_request(&mut self, sender: Token, header: &str, data: &Value) -> String {
        match header {
            _ => String::from(""),
        }
    }
}
