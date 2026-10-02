struct Lobby {
    name: String,
    list_players: Vec<String>,
    in_game: bool,
}

pub struct Lobbies {
    list: Vec<Lobby>,
}

impl Lobbies {
    pub fn new() -> Lobbies {
        Lobbies { list: Vec::new() }
    }
}
