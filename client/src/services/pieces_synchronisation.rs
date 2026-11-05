use crate::services::move_json::Move;
use serde::{Deserialize, Serialize};

/*
CLIENT SIDE PIECES JSON EXAMPLE
{
        "name": "pion",
        "index": 1,
        "cost":2,
        "label":"♙",
        "custom_label":"P",
        "team":0,
        "is_king": false
},

SERVER SIDE PIECES JSON EXAMPLE
{
        "name": "dame",
        "index": 5,
        "moves": [
            { "x": 1, "y": 1 },
            { "x": -1, "y": 1 },
            { "x": 1, "y": -1 },
            { "x": -1, "y": -1 },
            { "x": 1, "y": 0 },
            { "x": 0, "y": 1 },
            { "x": 0, "y": -1 },
            { "x": -1, "y": 0 }
        ],
        "extendable":true,
        "cost":9,
        "label":"♕",
        "eating_moves": [],
        "team":0,
        "is_king": false
    },
*/

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct ClientPiece {
    name: String,
    index: i8,
    cost: i8,
    label: char,
    custom_label: char,
    team: i8,
    is_king: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct ServerPiece {
    name: String,
    index: i8,
    moves: Vec<Move>,
    extendable: bool,
    cost: i8,
    label: String,
    eating_moves: Vec<Move>,
    team: i8,
    is_king: bool,
}

fn get_pieces_from_json(json: &str) -> Result<Vec<ServerPiece>, serde_json::Error> {
    serde_json::from_str(json)
}

fn get_pieces_from_file(json_file: &str) -> Result<Vec<ClientPiece>, Box<dyn std::error::Error>> {
    let pieces_json = std::fs::read_to_string(json_file)?;
    Ok(serde_json::from_str::<Vec<ClientPiece>>(&pieces_json)?)
}

fn to_client_piece(server_piece: &ServerPiece, current_piece: Option<&ClientPiece>) -> ClientPiece {
    let label = server_piece
        .label
        .chars()
        .next()
        .unwrap_or(' ');

    let custom_label = current_piece
        .map(|piece| piece.custom_label)
        .unwrap_or(label);

    ClientPiece {
        name: server_piece.name.clone(),
        index: server_piece.index,
        cost: server_piece.cost,
        label,
        custom_label,
        team: server_piece.team,
        is_king: server_piece.is_king,
    }
}

//gets the current client pieces from the json file and the server pieces from the json string, then synchronises them and writes the new client pieces to the json file
pub fn synchronise_pieces(pieces: String, json_file: &str) {
    let current_client_pieces = match get_pieces_from_file(json_file) { // get the current client pieces from the json file
        Ok(pieces) => pieces,
        Err(error) => {
            eprintln!("Impossible de lire les pièces client: {error}");
            return;
        }
    };

    // get the server pieces from the json string
    let server_pieces = match get_pieces_from_json(&pieces) { // get the server pieces from the json string
        Ok(pieces) => pieces,
        Err(error) => {
            eprintln!("Impossible de lire les pièces serveur: {error}");
            return;
        }
    };


    // Synchronise the pieces by creating a new vector of client pieces based on the server pieces and the current client pieces (lets the user keep their custom labels)
    let new_client_pieces: Vec<ClientPiece> = server_pieces
        .iter()
        .map(|server_piece| {
            let current_piece = current_client_pieces
                .iter()
                .find(|piece| piece.index == server_piece.index);

            to_client_piece(server_piece, current_piece)
        })
        .collect();

    if let Err(error) = std::fs::write(
        json_file,
        serde_json::to_string_pretty(&new_client_pieces).unwrap(),
    ) {
        eprintln!("Impossible d'écrire les pièces client: {error}");
    }
}