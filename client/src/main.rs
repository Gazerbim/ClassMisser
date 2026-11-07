pub mod chat;
pub mod client;
pub mod game;
pub mod menus;
mod metagame;
pub mod services;
pub mod types;

use crate::metagame::Metagame;
use crate::services::pieces_synchronisation;

use clap::Parser;

const PIECES_JSON: &str = "src/data/pieces.json";

#[derive(Parser)]
struct Cli {
    username: String,
    password: String,

    #[arg(long, default_value = "127.0.0.1:9001")]
    host: String,

    #[arg(long)]
    action: Option<String>,
}

fn synchronise_pieces(game: &mut Metagame) {
    let request = String::from("{ \"type\": \"send_pieces\", \"header\": \"send_pieces\" }$");
    if !game.get_client().send(&request) {
        eprintln!("Impossible d'envoyer la demande de pièces au serveur");
        return;
    }

    let start = std::time::Instant::now();
    let message = loop {
        if let Some(m) = game.get_client().receive_message() {
            break m;
        }
        if start.elapsed() > std::time::Duration::from_secs(5) {
            eprintln!("Pas de réponse du serveur pour la synchronisation des pièces");
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    };

    let response: serde_json::Value = match serde_json::from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Réponse pièces invalide: {e}");
            return;
        }
    };
    let pieces = match &response["data"]["pieces"] {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => {
            eprintln!("Réponse pièces sans champ data.pieces: {message}");
            return;
        }
        other => other.to_string(),
    };
    pieces_synchronisation::synchronise_pieces(pieces, PIECES_JSON);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut game = Metagame::new(&String::from(args.host));
    //connect
    let connected: bool = match args.action {
        Some(_a) => {
            if _a == "signup".to_string() {
                game.connect(args.username, args.password, "new_account")
            } else if _a == "signin" {
                game.connect(args.username, args.password, "connect")
            } else {
                println!("Cette option n'est pas disponnible... Essayez 'signup' ou 'signin'.");
                false
            }
        }
        None => game.connect(args.username, args.password, "connect"),
    };
    println!("tried to connect");
    if connected {
        synchronise_pieces(&mut game);

        game.main_loop();
    } else {
        println!("Impossible de lancer le jeux...");
    }
    Ok(())
}
