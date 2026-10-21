pub mod chat;
pub mod client;
pub mod game;
pub mod menus;
mod metagame;
pub mod services;
pub mod types;

use crate::metagame::Metagame;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    username: String,
    password: String,

    #[arg(long, default_value = "127.0.0.1:9001")]
    host: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut game = Metagame::new(&String::from(args.host));
    if game.connect(args.username, args.password) {
        game.main_loop();
    } else {
        println!("Le nom d'utilisateur ou le mot de passe ne sont pas correctes !");
    }
    Ok(())
}
