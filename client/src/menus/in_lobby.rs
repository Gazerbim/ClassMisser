use console_engine::{ConsoleEngine, KeyCode};

pub struct InLobby {
    name: String,
    creator: bool,
    player_number: i32,
    try_launch_game: bool,
}

impl InLobby {
    pub fn new(name: String) -> InLobby {
        InLobby {
            name: name,
            creator: false,
            player_number: 4,
            try_launch_game: false,
        }
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name.clone();
    }

    pub fn set_creator(&mut self, privilage: bool) {
        self.creator = privilage;
    }

    pub fn get_player_number(&self) -> i32 {
        self.player_number
    }

    pub fn get_try_launch_game(&self) -> bool {
        self.try_launch_game
    }

    pub fn handling_events(&mut self, e: &ConsoleEngine) {
        if self.creator {
            if e.is_key_pressed(KeyCode::Right) && self.player_number < 4 {
                self.player_number += 1;
            }
            if e.is_key_pressed(KeyCode::Left) && self.player_number > 2 {
                self.player_number -= 1;
            }
            if e.is_key_pressed(KeyCode::Enter) {
                self.try_launch_game = true;
            }
        }
    }

    pub fn update(&mut self) {
        self.try_launch_game = false;
    }

    pub fn display(&self, e: &mut console_engine::ConsoleEngine, top_left: (i32, i32)) {
        let mut text = "Dans le salon nommé ".to_string();
        text.push_str(&self.name);
        text.push_str(" !");
        e.print(top_left.0, top_left.1, &text);
        if self.creator {
            let msg =
                "Séléctionner le nombre de joueurs (les autres seront spectateurs):".to_string();
            let mut number_players = "< ".to_string();
            number_players.push_str(&self.player_number.to_string());
            number_players.push_str(&" >");
            e.print(
                top_left.0 + (text.len() / 2) as i32 - (msg.len() / 2) as i32,
                top_left.1 + 3,
                &msg,
            );
            e.print(
                top_left.0 + (text.len() / 2) as i32 - 3,
                top_left.1 + 5,
                &number_players,
            );
        }
    }
}
