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
    // true une fois que le joueur qui joue a move une piece pdt son tour.
    // tour = 1 déplacement, puis (optionnellement) 1 pièce à faire apparaître.
    has_moved: bool,
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
            has_moved: false,
        }
    }

    pub fn empty() -> Game {
        Game {
            board: Board::new_empty(),
            pieces: Pieces::new(),
            team_number: 0,
            player_playing: 0,
            players_points: [0, 0, 0, 0],
            has_moved: false,
        }
    }

    /// passe la main au joueur suivant.
    fn end_turn(&mut self) {
        self.has_moved = false;
        self.player_playing = (self.player_playing + 1) % self.team_number;
    }

    /// message sans données (par ex un refus) à destination d'un seul joueur
    fn simple_message(header: &str) -> String {
        serde_json::json!({
            "type": "game",
            "header": header,
            "data": {}
        })
        .to_string()
            + "$"
    }

    /// true si la case contient une pièce appartenant à un player en particulier
    fn is_piece_of(&self, cell: i8, player: i8) -> bool {
        let count = self.pieces.get_number_of_different_pieces() as i32;
        let local = cell as i32 - player as i32 * count;
        cell > 0 && local >= 1 && local <= count
    }

    pub fn handle_request(
        &mut self,
        sender: i8,
        header: &str,
        data: &Value,
    ) -> (i8, String) {
        match header {
            "piece_moves" => {
                let (x, y) = match (
                    data["x"].as_u64(),
                    data["y"].as_u64(),
                ) {
                    (Some(x), Some(y)) => (x as usize, y as usize),
                    _ => return (-1, String::new()),
                };

                if x >= self.board.board_size()
                    || y >= self.board.board_size()
                {
                    return (-1, String::new());
                }
                //bug du unpack 'id not in pieces' corrige
                if sender != self.player_playing
                    || self.has_moved
                    || !self.is_piece_of(self.board.get(x, y), sender)
                {
                    return (-1, String::new());
                }

                let moves = self.get_moves_for_piece(x, y);
                let mut xs = Vec::new();
                let mut ys = Vec::new();

                for m in moves {
                    xs.push(m.x);
                    ys.push(m.y);
                }

                let message = serde_json::json!({
                    "type": "game",
                    "header": "piece_moves",
                    "data": {
                        "x": xs,
                        "y": ys
                    }
                })
                .to_string() + "$";

                (sender, message)
            }

            "move_piece" => {
                let (ox, oy, nx, ny) = match (
                    data["ox"].as_u64(),
                    data["oy"].as_u64(),
                    data["nx"].as_u64(),
                    data["ny"].as_u64(),
                ) {
                    (Some(ox), Some(oy), Some(nx), Some(ny)) => (
                        ox as usize,
                        oy as usize,
                        nx as usize,
                        ny as usize,
                    ),
                    _ => return (-1, String::new()),
                };

                let size = self.board.board_size();

                if ox >= size || oy >= size || nx >= size || ny >= size {
                    return (-1, String::new());
                }

                //seul le joueur actif peut deplacer une piece
                if sender != self.player_playing {
                    return (-1, String::new());
                }

                if self.has_moved {
                    return (sender, Self::simple_message("move_rejected"));
                }

                let (x, y, p) = self.try_move_piece(
                    ox as i8,
                    oy as i8,
                    nx as i8,
                    ny as i8,
                    sender,
                );

                if x.is_empty() || y.is_empty() || p.is_empty() {
                    // coup illégal : on previent le joueur pour qu'il puisse rejouer.
                    return (sender, Self::simple_message("move_rejected"));
                }

                self.has_moved = true;

                // le joueur doit faire apparaitre une piece sauf s'il a pas assez de thunasse
                if !self
                    .pieces
                    .can_afford_any(self.players_points[sender as usize])
                {
                    self.end_turn();
                }

                let message = serde_json::json!({
                    "type": "game",
                    "header": "update_board",
                    "data": {
                        "x": x,
                        "y": y,
                        "pieces": p,
                        "player_playing": self.player_playing,
                        "players_points": self.players_points,
                    }
                })
                .to_string() + "$";

                (-1, message)
            }

            "spawn_piece" => {
                let reject = || {
                    let message = serde_json::json!({
                        "type": "game",
                        "header": "spawn_rejected",
                        "data": {}
                    }).to_string() + "$";

                    (sender, message)
                };

                let (piece, x, y) = match (
                    data["piece"].as_i64(),
                    data["x"].as_u64(),
                    data["y"].as_u64(),
                ) {
                    (Some(p), Some(x), Some(y)) => (p as i8, x as usize, y as usize),
                    _ => return reject(),
                };

                if sender != self.player_playing
                    || sender < 0
                    || sender >= self.team_number
                    || !self.has_moved
                {
                    return reject();
                }

                let size = self.board.board_size();
                let count = self.pieces.get_number_of_different_pieces();

                if piece < 1
                    || piece > count
                    || x >= size
                    || y >= size
                    || self.board.get(x, y) != 0
                {
                    return reject();
                }

                let Some(cost) = self.pieces.get_piece_cost(piece) else {
                    return reject();
                };

                if cost < 0 || self.players_points[sender as usize] < cost as usize {
                    return reject();
                }

                let king_id = match sender {
                    0 => -5,
                    1 => -4,
                    2 => -3,
                    3 => -2,
                    _ => return reject(),
                };

                let mut king_position = None;

                for ky in 0..size {
                    for kx in 0..size {
                        if self.board.get(kx, ky) == king_id {
                            king_position = Some((kx, ky));
                            break;
                        }
                    }

                    if king_position.is_some() {
                        break;
                    }
                }

                let Some((kx, ky)) = king_position else {
                    return reject();
                };

                let dx = (x as isize - kx as isize).abs();
                let dy = (y as isize - ky as isize).abs();

                if dx > 1 || dy > 1 || (dx == 0 && dy == 0) {
                    return reject();
                }

                let piece_id = piece + sender * count;

                // modification de l'etat uniquement après validation
                self.players_points[sender as usize] -= cost as usize;
                self.board.set(x, y, piece_id);
                self.end_turn();

                let message = serde_json::json!({
                    "type": "game",
                    "header": "update_board",
                    "data": {
                        "x": [x],
                        "y": [y],
                        "pieces": [piece_id],
                        "player_playing": self.player_playing,
                        "players_points": self.players_points
                    }
                }).to_string() + "$";

                (-1, message)
            }

            // skip spawn du player
            "skip_spawn" => {
                if sender != self.player_playing || !self.has_moved {
                    return (-1, String::new());
                }

                self.end_turn();

                let message = serde_json::json!({
                    "type": "game",
                    "header": "update_board",
                    "data": {
                        "x": Vec::<usize>::new(),
                        "y": Vec::<usize>::new(),
                        "pieces": Vec::<i8>::new(),
                        "player_playing": self.player_playing,
                        "players_points": self.players_points
                    }
                })
                .to_string()
                    + "$";

                (-1, message)
            }

            _ => (-1, String::new()),
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
        if self.is_piece_of(id, player)
        {
            //la pièce appartient bien au joueur
            println!("player {} is playing et la piece appartient au joueur", player);
            let moves =
                self.pieces
                    .get_moves_from_position(ox as usize, oy as usize, id, &self.board);
            for m in moves {
                if m.x == nx && m.y == ny {
                    // Le déplacement est légal.
                    self.board.move_piece(
                        ox as usize,
                        oy as usize,
                        nx as usize,
                        ny as usize,
                    );

                    //si on mange une piece, on ajoute les points du joueur
                    let eaten_piece = self.board.get(nx as usize, ny as usize);
                    if eaten_piece > 0 {
                        let eaten_piece_cost = self.pieces.get_piece_cost(eaten_piece).unwrap_or(0);
                        self.players_points[player as usize] += eaten_piece_cost as usize;
                    }

                    // Chaque déplacement légal rapporte un point.
                    self.players_points[player as usize] += 1;

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

    fn is_spawn_position_valid(
        &self,
        player: i8,
        x: usize,
        y: usize,
    ) -> bool {
        let size = self.board.board_size();

        if player < 0
            || player >= self.team_number
            || x >= size
            || y >= size
            || self.board.get(x, y) != 0
        {
            return false;
        }

        // Les rois sont représentés par -5, -4, -3 et -2.
        let king_id = -(5 - player);

        // Recherche du roi sur le plateau
        let king = (0..size).find_map(|ky| {
            (0..size)
                .find(|&kx| self.board.get(kx, ky) == king_id)
                .map(|kx| (kx, ky))
        });

        let Some((kx, ky)) = king else {
            return false;
        };

        let dx = (x as isize - kx as isize).abs();
        let dy = (y as isize - ky as isize).abs();

        // Les 8 cases voisines, diagonales comprises.
        dx <= 1 && dy <= 1 && (dx != 0 || dy != 0)
    }
}
