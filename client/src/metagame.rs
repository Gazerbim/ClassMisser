use console_engine::ConsoleEngine;

use crate::chat::Chat;
use crate::client::Client;
use crate::game::Game;
use crate::types::states::State;

use crate::menus::select_lobby::SelectLobby;

pub struct Metagame {
    client: Client,
    engine: ConsoleEngine,
    state: State,
    game: Game,
    chat: Chat,

    home: SelectLobby,
    lobby_selector: SelectLobby,
}

impl Metagame {
    pub fn new(host: &String) -> Metagame {
        Metagame {
            client: Client::new(host),
            engine: console_engine::ConsoleEngine::init_fill(20).unwrap(),
            state: State::Home,
            game: Game::new(),
            chat: Chat::new(),
            home: SelectLobby::new(vec![
                String::from("Créer un salon"),
                String::from("Rejoindre un salon"),
                String::from("Quitter"),
            ]),
            lobby_selector: SelectLobby::new(vec![String::from("Elie"), String::from("Serguei")]),
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            self.engine.wait_frame(); // wait for next frame + capture inputs
            self.engine.clear_screen(); // reset the screen

            if self.state == State::Home {
                let action = self.home.handling_events(&self.engine);
                if action == String::from("") {
                } else if action == "Quitter" {
                    break;
                } else if action == String::from("Rejoindre un salon") {
                    self.state = State::ChooseLobby;
                }
                self.home.update();
                self.home.display(&mut self.engine, (5, 5));
            } else if self.state == State::ChooseLobby {
                let action = &self.lobby_selector.handling_events(&self.engine);
                self.lobby_selector.display(&mut self.engine, (10, 5));
            } else if self.state == State::InGame {
                self.game.handling_hevents();
                self.chat.handling_events();
                self.game.update();
                self.chat.update();
                self.game.display();
                self.chat.display();
            }

            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                break; // exits app
            }
            self.engine.draw(); // draw the screen
        }
    }
}
