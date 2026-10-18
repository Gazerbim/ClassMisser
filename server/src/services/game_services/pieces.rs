use crate::services::game_services::{board::Board, move_json::Move, piece::Piece};
use std;

#[derive(Debug, Clone)]
pub struct Pieces {
    pieces: Vec<Piece>,
    team_number: i8,
    number_of_different_pieces: i8,
}

impl Pieces {
    pub fn new() -> Pieces {
        Pieces {
            pieces: Vec::new(),
            team_number: 0,
            number_of_different_pieces: 0,
        }
    }

    pub fn load_pieces(team_number: i8) -> Pieces {
        let content =
            std::fs::read_to_string("src/data/pieces.json").expect("error when reading the file");

        let pieces: Vec<Piece> = serde_json::from_str(&content).expect("error when parsing json");
        let number_of_different_pieces = pieces.len() as i8;

        let mut pieces_final: Vec<Piece> = Vec::new();

        for i in 0..team_number {
            for piece in &pieces {
                let mut npiece = piece.clone();
                npiece.team = i;
                npiece.index = piece.index + number_of_different_pieces * i;
                pieces_final.push(npiece);
            }
        }

        Pieces {
            pieces: pieces_final,
            team_number: team_number,
            number_of_different_pieces: number_of_different_pieces,
        }
    }

    pub fn get_number_of_different_pieces(&self) -> i8 {
        self.number_of_different_pieces
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

    pub fn get_moves_from_position(&self, x: usize, y: usize, id: i8, board: &Board) -> Vec<Move> {
        let piece = self.get_piece_from_id(id);
        let mut moves = Vec::new();

        for m in &piece.moves {
            let mut i: i8 = 0;
            let mut new_x = m.x + x as i8;
            let mut new_y = m.y + y as i8;

            while new_x >= 0
                && new_x < board.get_board_size() as i8
                && new_y >= 0
                && new_y < board.get_board_size() as i8
            {
                if (board.get(new_x as usize, new_y as usize) == -1)
                    || (board.is_occupied(new_x as usize, new_y as usize)
                        && self.get_team(board.get(new_x as usize, new_y as usize))
                            == self.get_team(board.get(x, y)))
                {
                    break;
                }

                moves.push(Move { x: new_x, y: new_y });

                if piece.extendable == false || board.is_occupied(new_x as usize, new_y as usize) {
                    break;
                }

                i += 1;
                new_x = m.x * i + x as i8;
                new_y = m.y * i + y as i8;
            }
        }

        for m in piece.get_eating_moves() {
            let new_x = m.x + x as i8;
            let new_y = m.y + y as i8;

            if new_x >= 0
                && new_x < board.get_board_size() as i8
                && new_y >= 0
                && new_y < board.get_board_size() as i8
            {
                if board.is_occupied(new_x as usize, new_y as usize)
                    && self.get_team(board.get(new_x as usize, new_y as usize))
                        != self.get_team(board.get_selected_piece_id())
                {
                    moves.push(Move { x: new_x, y: new_y });
                }
            }
        }

        return moves;
    }
}
