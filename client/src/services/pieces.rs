use crate::services::piece::Piece;
use std;

#[derive(Debug, Clone)]
pub struct Pieces {
    pieces: Vec<Piece>,
    team_number: i8,
}

impl Pieces {
    pub fn new() -> Pieces {
        Pieces {
            pieces: Vec::new(),
            team_number: 0,
        }
    }

    pub fn load_pieces(&mut self, team_number: i8) -> Pieces {
        let content =
            std::fs::read_to_string("src/data/pieces.json").expect("error when reading the file");

        let pieces: Vec<Piece> = serde_json::from_str(&content).expect("error when parsing json");

        let mut pieces_final: Vec<Piece> = Vec::new();

        for i in 0..team_number {
            for piece in &pieces {
                let mut npiece = piece.clone();
                npiece.team = i;
                npiece.index = piece.index + team_number * i;
                pieces_final.push(npiece);
            }
        }

        Pieces {
            pieces: pieces_final,
            team_number: team_number,
        }
    }

    fn get_piece_from_id(&self, id: i8) -> &Piece {
        self.pieces
            .iter()
            .find(|piece| piece.index == id)
            .expect("the id isnt in pieces")
    }

    fn get_team(&self, id: i8) -> i8 {
        return self.get_piece_from_id(id).team;
    }
}
