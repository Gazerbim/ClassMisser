use console_engine::ConsoleEngine;

use crate::chat::Chat;
use crate::client::Client;
use crate::game::Game;
use crate::menus::home::Home;
use crate::types::states::State;

use crate::menus::in_lobby::InLobby;
use crate::menus::select_lobby::SelectLobby;
use crate::menus::text_input::TextInput;

pub struct Metagame {
    client: Client,
    engine: ConsoleEngine,
    state: State,
    game: Game,
    chat: Chat,

    home: SelectLobby,
    home_layout: Home,
    enter_username: TextInput,
    lobby_selector: SelectLobby,
    lobby_creator: TextInput,
    in_lobby: InLobby,

    username: String,
}

impl Metagame {
    pub fn new(host: &String) -> Metagame {
        Metagame {
            client: Client::new(host),
            engine: console_engine::ConsoleEngine::init_fill(20).unwrap(),
            state: State::EnterPseudo,
            game: Game::new(),
            chat: Chat::new(),
            home: SelectLobby::new(vec![
                String::from("Créer un salon"),
                String::from("Rejoindre un salon"),
                String::from("Quitter"),
            ]),
            home_layout: Home::new(),
            enter_username: TextInput::new(String::from("Rentre to pseudo ici:"), 25),
            lobby_selector: SelectLobby::new(Vec::new()),
            lobby_creator: TextInput::new("Nom du salon à créer".to_string(), 25),
            in_lobby: InLobby::new("".to_string()),
            username: String::from(""),
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            self.engine.wait_frame(); // wait for next frame + capture inputs
            self.engine.clear_screen(); // reset the screen

            if self.handling_events() {
                break;
            }
            self.update();
            self.display();
        }
    }

    fn handling_events(&mut self) -> bool {
        if self.state == State::Home {
            let action = self.home.handling_events(&self.engine);
            if action == String::from("") {
            } else if action == "Quitter" {
                return true;
            } else if action == String::from("Rejoindre un salon") {
                //get la liste des lobby disponibles
                let lobby_name = self.client.request(&"test".to_string());
                self.lobby_selector
                    .set_elements_list(vec![String::from(lobby_name), String::from("Serguei")]);
                self.state = State::ChooseLobby;
            } else if action == "Créer un salon".to_string() {
                //requête pour créer un salon
                self.state = State::CreateLobby;
            }
        } else if self.state == State::EnterPseudo {
            self.enter_username.handling_events(&self.engine);
            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                return true; // exits app
            }
        } else if self.state == State::CreateLobby {
            self.lobby_creator.handling_events(&self.engine);
            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                //envoyer une requête pour quitter le lobby
                self.state = State::Home;
            }
        } else if self.state == State::InLobby {
            self.in_lobby.handling_events(&self.engine);
            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                //envoyer une requête pour quitter le lobby
                self.state = State::Home;
            }
        } else if self.state == State::ChooseLobby {
            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                self.state = State::Home;
            }
            let nom_lobby = self.lobby_selector.handling_events(&self.engine);
            if nom_lobby != "".to_string() {
                //demande connection au lobby
                // si demande acceptée, on rentre dans le lobby
                self.state = State::InLobby;
                self.in_lobby.set_creator(false);
                self.in_lobby.set_name(nom_lobby);
            }
        } else if self.state == State::InGame {
            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                //envoyer une requête pour prévenir qu'on a quitté la partie (voir ce qu'on fait dans ce cas)
                self.state = State::Home;
            }
            self.game.handling_hevents();
            self.chat.handling_events();
        }
        false
    }

    fn update(&mut self) {
        if self.state == State::Home {
            self.home.update();
            self.home_layout.update();
        } else if self.state == State::EnterPseudo {
            self.username = self.enter_username.update();
            if self.username != String::from("") {
                //envoyer au serveur le pseudo
                self.state = State::Home;
            }
        } else if self.state == State::ChooseLobby {
            self.lobby_selector.update();
        } else if self.state == State::CreateLobby {
            let new_lobby_name = self.lobby_creator.update();
            if new_lobby_name != String::from("") {
                //creer le lobby et le rejoindre
                self.in_lobby.set_name(new_lobby_name);
                self.in_lobby.set_creator(true);
                self.state = State::InLobby;
            }
        } else if self.state == State::InLobby {
            if self.in_lobby.get_try_launch_game() {
                //lancer la partie avec une requête
                self.state = State::InGame;
            }
            self.in_lobby.update();
        } else if self.state == State::InGame {
            self.game.update();
            self.chat.update();
        }
    }

    fn display(&mut self) {
        let left_third = ((self.engine.get_width() / 2) - 15) as i32;
        let top_left = (self.engine.get_height() / 4) as i32;
        if self.state == State::Home {
            self.home
                .display(&mut self.engine, (left_third, top_left + 10));
            self.home_layout
                .display(&mut self.engine, (left_third - 3, top_left * 4 / 15));
        } else if self.state == State::EnterPseudo {
            self.enter_username
                .display(&mut self.engine, (left_third, top_left));
        } else if self.state == State::ChooseLobby {
            self.lobby_selector
                .display(&mut self.engine, (left_third, top_left));
        } else if self.state == State::CreateLobby {
            self.lobby_creator
                .display(&mut self.engine, (left_third, top_left));
        } else if self.state == State::InLobby {
            self.in_lobby
                .display(&mut self.engine, (left_third, top_left));
        } else if self.state == State::InGame {
            self.game.display(&mut self.engine);
            self.chat.display();
        }
        self.engine.draw(); // draw the screen
    }
}
