use console_engine::ConsoleEngine;

pub struct Game {
    color: i32,
}

impl Game {
    pub fn new() -> Game {
        Game { color: -1 }
    }

    pub fn handling_hevents(&self) {}
    pub fn update(&self) {}
    pub fn display(&self, e: &mut ConsoleEngine) {
        e.print(10, 10, &"La partie est lancée !");
    }

    pub fn set_color(&mut self, new_color: i32) {
        self.color = new_color;
    }
}
