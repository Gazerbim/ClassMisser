use mio::Token;
use std::collections::HashMap;

pub struct Player {
    pseudo: String,
    pub lobby: Option<usize>,
}

impl Player {
    pub fn new(pseudo: String) -> Player {
        Player {
            pseudo: pseudo.clone(),
            lobby: None,
        }
    }

    pub fn set_lobby(&mut self, lobby: usize) {
        self.lobby = Some(lobby);
    }
}

pub struct Players {
    list: HashMap<Token, Player>,
}

impl Players {
    pub fn new() -> Players {
        Players {
            list: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, token: Token, player: Player) -> bool {
        //revoir la logique: vole le pseudo d'un autre joueur s'il existe déjà !!
        self.list.insert(token, player);
        true
    }

    pub fn get_player(&self, token: Token) -> Option<&Player> {
        self.list.get(&token)
    }
}
