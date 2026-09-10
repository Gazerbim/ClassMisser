pub mod chat;
pub mod client;
pub mod game;
pub mod menus;
mod metagame;
pub mod types;

use crate::metagame::Metagame;

fn main() {
    let mut game = Metagame::new(&String::from("127.0.0.1:9001"));
    game.main_loop();
}
