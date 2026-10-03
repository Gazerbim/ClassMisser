use mio::Token;

use crate::services::players;
pub struct Lobby {
    pub name: String,
    pub list_players: Vec<Token>,
    pub in_game: bool,
}

pub struct Lobbies {
    pub list: Vec<Lobby>,
}

impl Lobbies {
    pub fn new() -> Lobbies {
        Lobbies { list: Vec::new() }
    }

    pub fn add_player(&mut self, token:Token, lobby: usize){
        self.list[lobby].list_players.push(token);
    }
}
