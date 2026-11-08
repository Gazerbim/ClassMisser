use console_engine::{Color, ConsoleEngine, KeyCode};

use crate::services::save_game::SaveGame;

/// Lecteur de partie : permet de parcourir les plateaux d'une sauvegarde
/// image par image, avec les flèches ou h / l (comme dans vim).
pub struct Replay {
    name: String,
    save: Option<SaveGame>,
    frame: usize,
    error: Option<String>,
}

impl Replay {
    pub fn new() -> Replay {
        Replay { name: String::new(), save: None, frame: 0, error: None }
    }

    pub fn load(&mut self, name: &str, path: &str) {
        self.name = name.to_string();
        self.frame = 0;
        match SaveGame::from_file(path) {
            Ok(s) => { self.save = Some(s); self.error = None; }
            Err(e) => { self.save = None; self.error = Some(e.to_string()); }
        }
    }

    pub fn handling_events(&mut self, e: &ConsoleEngine) {
        let Some(save) = &self.save else { return };
        let last = save.len().saturating_sub(1);
        if e.is_key_pressed(KeyCode::Right) || e.is_key_pressed(KeyCode::Char('l')) {
            self.frame = (self.frame + 1).min(last);
        }
        if e.is_key_pressed(KeyCode::Left) || e.is_key_pressed(KeyCode::Char('h')) {
            self.frame = self.frame.saturating_sub(1);
        }
        // bonus vim : 0 / g = début, $ / G = fin
        if e.is_key_pressed(KeyCode::Char('0')) || e.is_key_pressed(KeyCode::Char('g')) {
            self.frame = 0;
        }
        if e.is_key_pressed(KeyCode::Char('$')) || e.is_key_pressed(KeyCode::Char('G')) {
            self.frame = last;
        }
    }

    pub fn display(&self, e: &mut ConsoleEngine, team_colors: &[Color; 4], texture: usize) {
        match &self.save {
            Some(save) if save.len() > 0 => {
                save.display(e, team_colors, texture, self.frame);
                let size = save.get_board(self.frame).map_or(0, |b| b.data.len()) as i32;
                let y = size * 3 + 1;
                e.print(0, y, &format!("{} - coup {}/{}", self.name, self.frame, save.len() - 1));
                e.print(0, y + 1, "h / <- : precedent | l / -> : suivant | g : debut | G : fin | Echap : retour");
            }
            Some(_) => e.print(0, 0, "Sauvegarde vide (Echap : retour)"),
            None => e.print(0, 0, &format!("Lecture impossible : {} (Echap : retour)", self.error.clone().unwrap_or_default())),
        }
    }
}
