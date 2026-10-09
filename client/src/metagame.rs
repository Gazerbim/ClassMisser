use console_engine::ConsoleEngine;
use serde_json::Value;

use crate::chat::Chat;
use crate::client::Client;
use crate::game::Game;
use crate::menus::home::Home;
use crate::types::protocol::{LobbiesList, Response};
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
                if self.client.send(
                    &"{\"type\": \"lobbies\",\"header\": \"list_available_lobbies\"}\n".to_string(),
                ) {
                    self.state = State::ChooseLobby;
                }
            } else if action == "Créer un salon".to_string() {
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
                let mut req =
                    "{\"type\": \"lobbies\",\"header\": \"quit_lobby\", \"data\": { \"name\": \""
                        .to_string();
                req.push_str(&self.in_lobby.get_name());
                req.push_str("\"}}\n");
                if self.client.send(&req) {
                    self.state = State::Home;
                }
            }
        } else if self.state == State::ChooseLobby {
            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                self.state = State::Home;
            }
            let nom_lobby = self.lobby_selector.handling_events(&self.engine);
            if nom_lobby != "".to_string() {
                //demande connection au lobby
                // si demande acceptée, on rentre dans le lobby
                let mut req =
                    "{\"type\": \"lobbies\",\"header\": \"enter_lobby\", \"data\": {\"name\": \""
                        .to_string();
                req.push_str(&nom_lobby);
                req.push_str("\" }}\n");
                if self.client.send(&req) {}
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
        self.read_server_responses();
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
                let mut req =
                    "{\"type\": \"lobbies\",\"header\": \"create_lobby\", \"data\": {\"name\": \""
                        .to_string();
                req.push_str(&new_lobby_name);
                req.push_str("\" }}\n");
                self.client.send(&req);
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
        self.engine.clear_screen(); // reset the screen
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

    fn read_server_responses(&mut self) {
        let mut raw_response: String = String::from("");
        if self.client.receive(&mut raw_response) {
            if raw_response != String::from("") {
                let response: Response = self.client.parse_response(&raw_response);
                match response.request_type.as_str() {
                    "lobbies" => self.read_lobby_server_response(response),
                    _ => {}
                }
            }
        }
    }

    fn read_lobby_server_response(&mut self, response: Response) {
        match response.header.as_str() {
            "list_available_lobbies" => match response.data["lobbies"].as_array() {
                Some(_v) => {
                    let mut v: Vec<String> = Vec::new();
                    for e in _v {
                        match e.as_str() {
                            Some(_s) => v.push(String::from(_s)),
                            None => {}
                        }
                    }
                    self.lobby_selector.set_elements_list(v);
                }
                None => self.lobby_selector.set_elements_list(vec![]),
            },
            "enter_lobby" => match response.data["name"].as_str() {
                Some(_n) => {
                    self.state = State::InLobby;
                    self.in_lobby.set_creator(false);
                    self.in_lobby.set_name(String::from(_n));
                }
                None => { //set le nom du lobby en rouge pour signifier une erreur
                }
            },
            "created_lobby" => match response.data["name"].as_str() {
                Some(_n) => {
                    self.in_lobby.set_name(String::from(_n));
                    self.in_lobby.set_creator(true);
                    self.state = State::InLobby;
                }
                None => { //set le nom du lobby en rouge pour signifier une erreur
                }
            },
            "grant_ownership" => match response.data["name"].as_str() {
                Some(_n) => {
                    if self.in_lobby.get_name() == _n[1..(_n.len() - 1)].to_string() {
                        self.in_lobby.set_creator(true);
                    } else {
                        //envoyer une erreur au serveur
                    }
                }
                None => {}
            },
            _ => {}
        }
    }
}
