use console_engine::ConsoleEngine;

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn handling_hevents(&self) {}
    pub fn update(&self) {}
    pub fn display(&self, e: &mut ConsoleEngine) {
        e.print(10, 10, &"La partie est lancée !");
    }
}
