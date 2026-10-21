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

    #[arg(long)]
    action: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut game = Metagame::new(&String::from(args.host));
    //connect
    let connected: bool = match args.action {
        Some(_a) => {
            if _a == "signup".to_string() {
                //créer un compte
                false
            } else if _a == "signin" {
                game.connect(args.username, args.password)
            } else {
                println!("Cette option n'est pas disponnible... Essayez 'signup' ou 'signin'.");
                false
            }
        }
        None => game.connect(args.username, args.password),
    };

    if connected {
        game.main_loop();
    } else {
        println!("Impossible de lancer le jeux...");
    }
    Ok(())
}
