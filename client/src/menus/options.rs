use std::collections::HashMap;

use console_engine::ConsoleEngine;
use serde_json::Value;

pub struct Options {
    textures: Vec<String>,
    chosen_texture: usize,
    texts: Vec<String>,
    focus: i32,
    number_of_parameters: i32,
    validate: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            textures: vec!["classique".to_string(), "aveugle".to_string()],
            chosen_texture: 0,
            texts: vec!["Quelle texture voulez-vous utiliser ?".to_string()],
            focus: 0,
            number_of_parameters: 1,
            validate: false,
        }
    }

    pub fn handling_events(&mut self, e: &ConsoleEngine) {
        if e.is_key_pressed(console_engine::KeyCode::Down) {
            self.focus -= 1;
            if self.focus < -1 {
                self.focus = -1;
            }
        }
        if e.is_key_pressed(console_engine::KeyCode::Up) {
            self.focus += 1;
            if self.focus >= self.number_of_parameters {
                self.focus = self.number_of_parameters - 1;
            }
        }
        if self.focus == 0 {
            if e.is_key_pressed(console_engine::KeyCode::Right) {
                self.chosen_texture += 1;
                if self.chosen_texture >= self.textures.len() {
                    self.chosen_texture = 0;
                }
            }
            if e.is_key_pressed(console_engine::KeyCode::Left) {
                if self.chosen_texture == 0 {
                    self.chosen_texture = self.textures.len() - 1;
                } else {
                    self.chosen_texture -= 1;
                }
            }
        } else {
            if e.is_key_pressed(console_engine::KeyCode::Enter) {
                self.validate = true;
            }
        }
    }

    pub fn update(&mut self) -> Option<HashMap<String, usize>> {
        if self.validate {
            self.validate = false;
            let mut hash: HashMap<String, usize> = HashMap::new();
            hash.insert("textures".to_string(), self.chosen_texture);
            return Some(hash);
        }
        None
    }

    pub fn display(&self, e: &mut ConsoleEngine, top_left: (i32, i32)) {
        e.print(top_left.0, top_left.1, "Paramètres:");

        if self.focus >= 0 {
            e.print(top_left.0 - 2, top_left.1 + 3 + (self.focus) * 5, ">");
        } else {
            e.print(
                top_left.0 - 2,
                top_left.1 + (self.number_of_parameters) * 5 + 3,
                ">",
            );
        }

        e.print(top_left.0, top_left.1 + 3, &self.texts[0]);
        e.print(
            top_left.0,
            top_left.1 + 5,
            &self.textures[self.chosen_texture],
        );

        e.print(
            top_left.0,
            top_left.1 + (self.number_of_parameters) * 5 + 3,
            "Valider",
        );
    }
}
