use std::fs;

use array2d::Array2D;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::game_services::board::Board;
use crate::services::game_services::move_json::Move;
use crate::services::game_services::pieces::Pieces;
use crate::services::players;
//
// const TEAM_NUMBER: i8 = 4;
// const BOARD_SIZE: usize = 8;
//
// #[derive(Debug, Deserialize, Clone)]
// struct Move {
//     x: i8,
//     y: i8,
// }
//
// #[derive(Debug, Deserialize, Clone)]
// struct Piece {
//     name: String,
//     index: i8,
//     moves: Vec<Move>,
//     extendable: bool,
//     cost: i8,
//     label: String,
//     team: i8,
//     eating_moves: Vec<Move>,
// }
//
// impl Piece {
//     fn new(
//         name: String,
//         index: i8,
//         moves: Vec<Move>,
//         extendable: bool,
//         cost: i8,
//         label: String,
//         team: i8,
//         eating_moves: Vec<Move>,
//     ) -> Piece {
//         Piece {
//             name: name,
//             index: index,
//             moves: moves,
//             extendable: extendable,
//             cost: cost,
//             label: label,
//             team: team,
//             eating_moves: eating_moves,
//         }
//     }
// }
//
// #[derive(Debug, Clone)]
// struct Pieces {
//     pieces: Vec<Piece>,
// }
//
// impl Pieces {
//     fn load_pieces() -> Pieces {
//         let content = fs::read_to_string("src/pieces.json").expect("error when reading the file");
//
//         let pieces: Vec<Piece> = serde_json::from_str(&content).expect("error when parsing json");
//
//         let mut pieces_final: Vec<Piece> = Vec::new();
//
//         for i in 0..TEAM_NUMBER {
//             for piece in &pieces {
//                 let mut npiece = piece.clone();
//                 npiece.team = i;
//                 npiece.index = piece.index + TEAM_NUMBER * i;
//                 pieces_final.push(npiece);
//             }
//         }
//
//         Pieces {
//             pieces: pieces_final,
//         }
//     }
//
//     fn get_piece_from_id(&self, id: i8) -> &Piece {
//         self.pieces
//             .iter()
//             .find(|piece| piece.index == id)
//             .expect("the id isnt in pieces")
//     }
//
//     fn get_team(&self, id: i8) -> i8 {
//         return self.get_piece_from_id(id).team;
//     }
//
//     fn get_moves_from_position(&self, x: usize, y: usize, id: i8, board: &Board) -> Vec<Move> {
//         let piece = self.get_piece_from_id(id);
//         let mut moves = Vec::new();
//
//         for m in &piece.moves {
//             let mut i: i8 = 0;
//             let mut new_x = m.x + x as i8;
//             let mut new_y = m.y + y as i8;
//
//             while new_x >= 0 && new_x < BOARD_SIZE as i8 && new_y >= 0 && new_y < BOARD_SIZE as i8 {
//                 if (board.get(new_x as usize, new_y as usize) == -1)
//                     || (board.is_occupied(new_x as usize, new_y as usize)
//                         && self.get_team(board.get(new_x as usize, new_y as usize))
//                             == self.get_team(board.get(x, y)))
//                 {
//                     break;
//                 }
//
//                 moves.push(Move { x: new_x, y: new_y });
//
//                 if piece.extendable == false || board.is_occupied(new_x as usize, new_y as usize) {
//                     break;
//                 }
//
//                 i += 1;
//                 new_x = m.x * i + x as i8;
//                 new_y = m.y * i + y as i8;
//             }
//         }
//
//         for m in &piece.eating_moves {
//             let new_x = m.x + x as i8;
//             let new_y = m.y + y as i8;
//
//             if new_x >= 0 && new_x < BOARD_SIZE as i8 && new_y >= 0 && new_y < BOARD_SIZE as i8 {
//                 if board.is_occupied(new_x as usize, new_y as usize)
//                     && self.get_team(board.get(new_x as usize, new_y as usize))
//                         != self.get_team(board.get(x, y))
//                 {
//                     moves.push(Move { x: new_x, y: new_y });
//                 }
//             }
//         }
//
//         return moves;
//     }
// }
//
// fn serialize_array<S>(array: &Array2D<i8>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let rows: Vec<Vec<i8>> = (0..array.num_rows())
//         .map(|r| {
//             (0..array.num_columns())
//                 .map(|c| *array.get(r, c).unwrap())
//                 .collect()
//         })
//         .collect();
//
//     rows.serialize(serializer)
// }
//
// #[derive(Serialize)]
// struct Board {
//     #[serde(serialize_with = "serialize_array")]
//     data: Array2D<i8>,
// }
//
// impl Board {
//     fn new() -> Board {
//         let grid: Array2D<i8> = Array2D::filled_with(0, BOARD_SIZE, BOARD_SIZE);
//         Board { data: grid }
//     }
//
//     fn set(&mut self, x: usize, y: usize, value: i8) {
//         self.data[(y, x)] = value;
//     }
//
//     fn get(&self, x: usize, y: usize) -> i8 {
//         return self.data[(y, x)];
//     }
//
//     //move piece from one point to another
//     fn move_piece(&mut self, x_from: usize, y_from: usize, x_to: usize, y_to: usize) -> i8 {
//         let what_is_to = self.get(x_to, y_to);
//         self.set(x_to, y_to, self.get(x_from, y_from));
//         self.set(x_from, y_from, 0);
//
//         return what_is_to;
//     }
//
//     //returns wether the cell is occupied by a piece
//     fn is_occupied(&self, x: usize, y: usize) -> bool {
//         return self.get(x, y) > 0;
//     }
// }

pub struct Game {
    pub board: Board,
    pub pieces: Pieces,
    pub team_number: i8,
    player_playing: i8,
    players_points: [usize; 4],
}

impl Game {
    pub fn new(team_number: i8, board_size: usize) -> Game {
        let pieces = Pieces::load_pieces(team_number);
        Game {
            board: Board::new(team_number, board_size, &pieces),
            pieces: pieces,
            team_number: team_number,
            player_playing: 0,
            players_points: [0, 0, 0, 0],
        }
    }

    pub fn empty() -> Game {
        Game {
            board: Board::new_empty(),
            pieces: Pieces::new(),
            team_number: 0,
            player_playing: 0,
            players_points: [0, 0, 0, 0],
        }
    }

    pub fn handle_request(&mut self, sender: i8, header: &str, data: &Value) -> (i8, String) {
        match header {
            "piece_moves" => match data["x"].as_i64() {
                Some(_x) => match data["y"].as_i64() {
                    Some(_y) => {
                        let moves = self.get_moves_for_piece(_x as usize, _y as usize);
                        let mut x_pos = Vec::new();
                        let mut y_pos = Vec::new();
                        for m in moves {
                            x_pos.push(m.x);
                            y_pos.push(m.y);
                        }
                        let message = serde_json::json!({
                            "type": "game",
                            "header": "piece_moves",
                            "data": {
                                "x": x_pos,
                                "y": y_pos
                            }
                        })
                        .to_string()
                            + "$";
                        println!("send to: {}", sender);
                        println!("{}", message);
                        (sender, message)
                    }
                    None => (-1, "".to_string()),
                },
                None => (-1, "".to_string()),
            },
            "move_piece" => match (
                data["ox"].as_i64(),
                data["oy"].as_i64(),
                data["nx"].as_i64(),
                data["ny"].as_i64(),
            ) {
                (Some(ox), Some(oy), Some(nx), Some(ny)) => {
                    if ox > 0 && oy > 0 && nx > 0 && ny > 0 {
                        let (x, y, p) =
                            self.try_move_piece(ox as i8, oy as i8, nx as i8, ny as i8, sender);
                        if x.len() == 0 || y.len() == 0 || p.len() == 0 {
                            println!("invalid move, no possible moves");
                            return (-1, "".to_string());
                        }
                        self.player_playing = (self.player_playing + 1) % self.team_number;
                        
                        return(
                            -1,
                            serde_json::json!({
                                "type": "game",
                                "header": "update_board",
                                "data": {
                                    "x": x,
                                    "y": y,
                                    "pieces": p,
                                    "player_playing": self.player_playing,
                                }
                            })
                            .to_string()
                                + "$",
                        );
                        
                    } else {
                        println!("invalid move, out of bounds");
                        (-1, "".to_string())
                    }
                }
                (None, _, _, _) => (-1, "".to_string()),
                (_, None, _, _) => (-1, "".to_string()),
                (_, _, None, _) => (-1, "".to_string()),
                (_, _, _, None) => (-1, "".to_string()),
            },
            _ => (-1, "".to_string()),
        }
    }

    fn get_moves_for_piece(&self, x: usize, y: usize) -> Vec<Move> {
        let piece = self.board.get(x, y);
        self.pieces
            .get_moves_from_position(x, y, piece, &self.board)
    }

    fn try_move_piece(
        &mut self,
        ox: i8,
        oy: i8,
        nx: i8,
        ny: i8,
        player: i8,
    ) -> (Vec<usize>, Vec<usize>, Vec<i8>) {
        //vérifier légalité mouvement
        let id = self.board.get(ox as usize, oy as usize);
        println!(" id {} - player {}*{}", id, player, self.pieces.get_number_of_different_pieces());
        if id - player * self.pieces.get_number_of_different_pieces() >= 1
        
        {
            //la pièce appartient bien au joueur
            println!("player {} is playing et la piece appartient au joueur", player);
            let moves =
                self.pieces
                    .get_moves_from_position(ox as usize, oy as usize, id, &self.board);
            for m in moves {
                if m.x == nx && m.y == ny {
                    //move effectivement possible
                    let eaten =
                        self.board
                            .move_piece(ox as usize, oy as usize, nx as usize, ny as usize);
                    if eaten > 0 {
                        self.players_points[self.player_playing as usize] += 1;
                    }
                    return (
                        vec![nx as usize, ox as usize],
                        vec![ny as usize, oy as usize],
                        vec![id, 0],
                    );
                }
            }
        }
        (Vec::new(), Vec::new(), Vec::new())
    }
}
