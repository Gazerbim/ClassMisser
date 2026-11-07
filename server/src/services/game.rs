use std::fs;

use array2d::Array2D;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::account_manager::AccountManager;
use crate::services::game_services::board::Board;
use crate::services::game_services::move_json::Move;
use crate::services::game_services::pieces::Pieces;
use crate::services::players;

pub struct Game {
    pub board: Board,
    pub pieces: Pieces,
    pub team_number: i8,
    player_playing: i8,
    players_points: [usize; 4],
    // true une fois que le joueur qui joue a move une piece pdt son tour.
    // tour = 1 déplacement, puis (optionnellement) 1 pièce à faire apparaître.
    has_moved: bool,
    king_eaten: i8, //>=0 si le joueur mange un roi lors de ce tour
    afk_players: Vec<bool>,
    player_won: i8,
    elo_changes: [i32; 4],
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
            king_eaten: -1,
            afk_players: vec![false, false, false, false],
            player_won: -1,
            elo_changes: [0, 0, 0, 0],
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
            king_eaten: -1,
            afk_players: vec![false, false, false, false],
            player_won: -1,
            elo_changes: [0, 0, 0, 0],
        }
    }

    pub fn set_player_afk_state(&mut self, to_change: usize, new_state: bool) {
        self.afk_players[to_change] = new_state;
    }

    pub fn get_player_afk_state(&self, player: usize) -> bool {
        self.afk_players[player]
    }

    pub fn get_simplified_board(&self) -> (Vec<usize>, Vec<usize>, Vec<i8>) {
        self.board.get_simplfied_board()
    }

    pub fn get_board_size(&self) -> usize {
        self.board.get_board_size()
    }

    pub fn get_player_playing(&self) -> i8 {
        self.player_playing
    }

    pub fn get_player_won(&self) -> i8 {
        self.player_won
    }

    pub fn get_elo_changes(&self) -> &[i32; 4] {
        &self.elo_changes
    }

    /// passe la main au joueur suivant.
    pub fn end_turn(&mut self) {
        println!("end turn");
        let mut non_blocking = 0;
        self.player_playing = (self.player_playing + 1) % self.team_number;
        while self.afk_players[self.player_playing as usize] && non_blocking < self.team_number * 2
        {
            self.player_playing = (self.player_playing + 1) % self.team_number;
            non_blocking += 1;
        }
        self.has_moved = false;
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

    pub fn handle_request(&mut self, sender: i8, header: &str, data: &Value) -> (i8, String) {
        match header {
            "piece_moves" => {
                let (x, y) = match (data["x"].as_u64(), data["y"].as_u64()) {
                    (Some(x), Some(y)) => (x as usize, y as usize),
                    _ => return (-1, String::new()),
                };

                if x >= self.board.board_size() || y >= self.board.board_size() {
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
                .to_string()
                    + "$";

                (sender, message)
            }

            "move_piece" => {
                let (ox, oy, nx, ny) = match (
                    data["ox"].as_u64(),
                    data["oy"].as_u64(),
                    data["nx"].as_u64(),
                    data["ny"].as_u64(),
                ) {
                    (Some(ox), Some(oy), Some(nx), Some(ny)) => {
                        (ox as usize, oy as usize, nx as usize, ny as usize)
                    }
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

                let (mut x, mut y, mut p) =
                    self.try_move_piece(ox as i8, oy as i8, nx as i8, ny as i8, sender);

                if x.is_empty() || y.is_empty() || p.is_empty() {
                    // coup illégal : on previent le joueur pour qu'il puisse rejouer.
                    return (sender, Self::simple_message("move_rejected"));
                } else {
                    if self.king_eaten >= 0 {
                        self.elo_changes[self.player_playing as usize] += 1000;
                        self.board.a_player_lost(
                            self.player_playing,
                            self.king_eaten,
                            &self.pieces,
                        );
                        self.afk_players[self.king_eaten as usize] = true; //si le joueur perd, il deviens afk, i.e. ne peut plus jouer
                        self.king_eaten = -1;
                        let (old_x, old_y, old_p) = (x, y, p);
                        (x, y, p) = self.board.get_simplfied_board();
                        x.extend(&old_x);
                        y.extend(&old_y);
                        p.extend(&old_p);
                    }
                }

                self.has_moved = true;

                // le joueur doit faire apparaitre une piece sauf s'il a pas assez de thunasse
                let mut special = "normal";
                if !self
                    .pieces
                    .can_afford_any(self.players_points[sender as usize])
                {
                    self.end_turn();
                    special = "skip_spawn"
                }

                let a_player_won = self.board.a_player_won(&self.pieces); // =-1 quand non
                if a_player_won != -1 {
                    self.player_won = a_player_won;
                    self.elo_changes[self.player_won as usize] += 3000;
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
                        "special_action": special, //pour quand le joueur n'a plus de pièces à bouger
                        "a_player_won": &a_player_won,
                        "elo_changes": self.elo_changes
                    }
                })
                .to_string()
                    + "$";

                (-1, message)
            }

            "spawn_piece" => {
                let reject = || {
                    let message = serde_json::json!({
                        "type": "game",
                        "header": "spawn_rejected",
                        "data": {}
                    })
                    .to_string()
                        + "$";

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
                    println!(
                        "spawn_piece: sender {} is not the player playing {} or has_moved {}",
                        sender, self.player_playing, self.has_moved
                    );
                    return reject();
                }

                let size = self.board.board_size();
                let count = self.pieces.get_number_of_different_pieces();

                if piece < 1 || piece > count || x >= size || y >= size || self.board.get(x, y) != 0
                {
                    println!(
                        "piece {} ou position ({},{}) invalide pour le joueur {}",
                        piece, x, y, sender
                    );
                    return reject();
                }

                let Some(cost) = self.pieces.get_piece_cost(piece) else {
                    println!("piece {} not found", piece);
                    return reject();
                };

                if cost < 0 || self.players_points[sender as usize] < cost as usize {
                    println!(
                        "player {} has {} points but piece {} costs {}",
                        sender, self.players_points[sender as usize], piece, cost
                    );
                    return reject();
                }

                let king_id = match sender {
                    0 => self.pieces.get_king_index(),
                    1 => self.pieces.get_team_king_id(1),
                    2 => self.pieces.get_team_king_id(2),
                    3 => self.pieces.get_team_king_id(3),
                    _ => return reject(),
                };

                let mut king_position = None;

                for ky in 0..size {
                    for kx in 0..size {
                        if self.board.get(kx, ky) == king_id {
                            king_position = Some((kx, ky));
                            println!(
                                "king (id: {}) found for player {} at position ({},{})",
                                king_id, sender, kx, ky
                            );
                            break;
                        }
                    }

                    if king_position.is_some() {
                        break;
                    }
                }

                let Some((kx, ky)) = king_position else {
                    println!("king not found for player {}", sender);
                    return reject();
                };

                let dx = (x as isize - kx as isize).abs();
                let dy = (y as isize - ky as isize).abs();

                if dx > 1 || dy > 1 || (dx == 0 && dy == 0) {
                    println!(
                        "spawn_piece: position ({},{}) is not adjacent to king at ({},{}) for player {}",
                        x, y, kx, ky, sender
                    );
                    return reject();
                }

                let piece_id = piece + sender * count;

                // modification de l'etat uniquement après validation
                self.players_points[sender as usize] -= cost as usize;
                self.board.set(x, y, piece_id);
                self.end_turn();

                println!(
                    "number of pieces ({}) of {}",
                    self.board
                        .get_number_alive_pieces_of_player(&self.pieces, self.player_playing),
                    self.player_playing,
                );
                //si le joueur n'a plus de pièce, il ne peut pas en déplacer
                let mut special = "normal";
                if self
                    .board
                    .get_number_alive_pieces_of_player(&self.pieces, self.player_playing)
                    <= 1
                //le roi est une pièce
                {
                    println!("effectivement plus de pièces");
                    self.has_moved = true;
                    self.players_points[self.player_playing as usize] += 1;
                    special = "buy_piece";
                }

                let message = serde_json::json!({
                    "type": "game",
                    "header": "update_board",
                    "data": {
                        "x": [x],
                        "y": [y],
                        "pieces": [piece_id],
                        "player_playing": self.player_playing,
                        "players_points": self.players_points,
                        "special_action": special
                    }
                })
                .to_string()
                    + "$";

                (-1, message)
            }

            // skip spawn du player
            "skip_spawn" => {
                if sender != self.player_playing || !self.has_moved {
                    return (-1, String::new());
                }

                self.end_turn();

                println!(
                    "number of pieces ({}) of {}",
                    self.board
                        .get_number_alive_pieces_of_player(&self.pieces, self.player_playing),
                    self.player_playing,
                );
                //si le joueur n'a plus de pièce, il ne peut pas en déplacer
                let mut special = "normal";
                if self
                    .board
                    .get_number_alive_pieces_of_player(&self.pieces, self.player_playing)
                    <= 1
                //le roi est une pièce
                {
                    println!("effectivement plus de pièces");
                    self.has_moved = true;
                    self.players_points[self.player_playing as usize] += 1;
                    special = "buy_piece";
                }

                let message = serde_json::json!({
                    "type": "game",
                    "header": "update_board",
                    "data": {
                        "x": Vec::<usize>::new(),
                        "y": Vec::<usize>::new(),
                        "pieces": Vec::<i8>::new(),
                        "player_playing": self.player_playing,
                        "players_points": self.players_points,
                        "special_action": special
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
        println!(
            " id {} - player {}*{}",
            id,
            player,
            self.pieces.get_number_of_different_pieces()
        );
        if self.is_piece_of(id, player) {
            //la pièce appartient bien au joueur
            println!(
                "player {} is playing et la piece appartient au joueur",
                player
            );
            let moves =
                self.pieces
                    .get_moves_from_position(ox as usize, oy as usize, id, &self.board);
            for m in moves {
                if m.x == nx && m.y == ny {
                    // Le déplacement est légal.
                    let eaten_piece = self.board.get(nx as usize, ny as usize);
                    self.board
                        .move_piece(ox as usize, oy as usize, nx as usize, ny as usize);

                    //si on mange une piece, on ajoute les points du joueur
                    if eaten_piece > 0 {
                        self.players_points[player as usize] += 1;
                        if self.pieces.get_piece_index(eaten_piece) == self.pieces.get_king_index()
                        {
                            self.king_eaten = self.pieces.get_team(eaten_piece);
                            //augmenter un peu elo joueur
                        }
                    }

                    // chaque déplacement rapporte un point
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

    fn is_spawn_position_valid(&self, player: i8, x: usize, y: usize) -> bool {
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

        // Les 8 cases voisines, diagonales comprises
        dx <= 1 && dy <= 1 && (dx != 0 || dy != 0)
    }
}
