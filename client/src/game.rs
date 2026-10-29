use console_engine::{Color, ConsoleEngine, KeyCode, pixel};
use serde_json::Value;

use crate::services::board::Board;
use crate::services::pieces::Pieces;
use crate::types::protocol::Response;
use crate::types::states::GameState;

pub struct Game {
    color: i32,
    board: Board,
    pieces: Pieces,
    is_playing: i32,
    state: GameState,
    team_colors: [Color; 4],
    players_points: [usize; 4],
    pending_request: String,
    selected_piece: usize,
    choosing_spawn: bool,
    spawn_positions: Vec<(usize, usize)>,
    selected_spawn: usize,
    previous_cursor: (usize, usize),
    awaiting_move: bool,
    team_number: i8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            color: -1,
            is_playing: -1,
            board: Board::new(),
            pieces: Pieces::new(),
            state: GameState::WaitInit,
            team_colors: [Color::Magenta, Color::Blue, Color::DarkYellow, Color::Green],
            players_points: [0; 4],
            pending_request: "".to_string(),
            selected_piece: 0,
            choosing_spawn: false,
            spawn_positions: Vec::new(),
            selected_spawn: 0,
            previous_cursor: (0, 0),
            awaiting_move: false,
            team_number: 4,
        }
    }

    pub fn set_color(&mut self, new_color: i32) {
        self.color = new_color;
    }

    pub fn clear_pending_request(&mut self) {
        self.pending_request = "".to_string();
    }

    pub fn init_game(
        &mut self,
        color: i32,
        board_size: usize,
        team_number: i8,
        x: &Vec<Value>,
        y: &Vec<Value>,
        pieces: &Vec<Value>,
    ) {
        let mut xc: Vec<usize> = Vec::new();
        let mut yc: Vec<usize> = Vec::new();
        let mut all_pieces: Vec<i8> = Vec::new();
        if x.len() == y.len() && y.len() == pieces.len() {
            for i in 0..x.len() {
                match x[i].as_i64() {
                    Some(_x) => match y[i].as_i64() {
                        Some(_y) => match pieces[i].as_i64() {
                            Some(_p) => {
                                xc.push(_x as usize);
                                yc.push(_y as usize);
                                all_pieces.push(_p as i8);
                            }
                            None => {}
                        },
                        None => {}
                    },
                    None => {}
                }
            }
        }
        self.pieces.load_pieces(team_number);
        self.board.init(board_size, team_number, xc, yc, all_pieces);
        self.color = color;
        self.is_playing = 0;
        self.team_number = team_number;

        // restore base state.
        self.players_points = [0; 4];
        self.selected_piece = 0;
        self.choosing_spawn = false;
        self.spawn_positions.clear();
        self.selected_spawn = 0;
        self.awaiting_move = false;
        self.pending_request = "".to_string();
        self.state = if self.color == self.is_playing {
            GameState::WaitPlay
        } else {
            GameState::WaitInit
        };
    }

    pub fn handle_server(&mut self, response: Response) {
        match response.header.as_str() {
            "piece_moves" => {
                let x: Option<Vec<usize>> = response.data["x"].as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as usize))
                        .collect()
                });

                let y: Option<Vec<usize>> = response.data["y"].as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as usize))
                        .collect()
                });

                if let (Some(x), Some(y)) = (x, y) {
                    if !x.is_empty() && x.len() == y.len() {
                        self.board.clear_highlights();
                        self.board.highlight(&x, &y);
                    }
                }
            }

            "update_board" => {
                let x = response.data["x"].as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as usize))
                        .collect::<Vec<_>>()
                });

                let y = response.data["y"].as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as usize))
                        .collect::<Vec<_>>()
                });

                let p = response.data["pieces"].as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_i64().map(|n| n as i8))
                        .collect::<Vec<_>>()
                });

                if let (Some(x), Some(y), Some(p)) = (x, y, p) {
                    if x.len() == y.len() && y.len() == p.len() {
                        self.board.update(x, y, p);
                    }
                }

                // Les points d'abord : le menu de choix de pièce en dépend.
                if let Some(points) = response.data["players_points"].as_array() {
                    for (i, point) in points.iter().enumerate().take(4) {
                        if let Some(value) = point.as_u64() {
                            self.players_points[i] = value as usize;
                        }
                    }
                }

                if let Some(player) = response.data["player_playing"].as_i64() {
                    if (0..=3).contains(&player) {
                        self.is_playing = player as i32;

                        if self.is_playing != self.color {
                            self.awaiting_move = false;
                            self.choosing_spawn = false;
                            self.spawn_positions.clear();
                            self.state = GameState::WaitEnd;
                        } else if self.awaiting_move {
                            // choisir la piece a faire spawner
                            self.awaiting_move = false;
                            self.selected_piece = 0;
                            self.state = GameState::WaitChoosePiece;
                        } else if self.state != GameState::WaitChoosePiece && !self.choosing_spawn {
                            self.state = GameState::WaitPlay;
                            match response.data["special_action"].as_str() {
                                Some("buy_piece") => {
                                    // force à choisir la piece a faire spawner
                                    self.awaiting_move = false;
                                    self.selected_piece = 0;
                                    self.state = GameState::WaitChoosePiece;
                                }
                                Some(_) => {}
                                None => {}
                            }
                        }
                    }
                }
            }

            "move_rejected" => {
                // Coup rejected -> rejouer
                self.awaiting_move = false;
                self.board.clear_highlights();
                self.state = GameState::WaitPlay;
            }

            "spawn_rejected" => {
                self.choosing_spawn = false;
                self.spawn_positions.clear();
                self.selected_spawn = 0;
                self.board
                    .set_selected(self.previous_cursor.0, self.previous_cursor.1);
                self.state = GameState::WaitChoosePiece;
            }

            _ => {}
        }
    }

    pub fn send_request(&self) -> Option<&String> {
        if self.pending_request != "".to_string() {
            Some(&self.pending_request)
        } else {
            None
        }
    }

    pub fn handling_hevents(&mut self, e: &mut ConsoleEngine, lobby_name: String) {
        // choisir une case de spawn pour la pièce sélectionnée
        if self.choosing_spawn {
            if self.spawn_positions.is_empty() {
                self.choosing_spawn = false;
                self.state = GameState::WaitChoosePiece;
                return;
            }

            let previous = self.selected_spawn;

            if e.is_key_pressed(KeyCode::Right) || e.is_key_pressed(KeyCode::Down) {
                self.selected_spawn = (self.selected_spawn + 1) % self.spawn_positions.len();
            } else if e.is_key_pressed(KeyCode::Left) || e.is_key_pressed(KeyCode::Up) {
                self.selected_spawn = (self.selected_spawn + self.spawn_positions.len() - 1)
                    % self.spawn_positions.len();
            }

            if previous != self.selected_spawn {
                let (x, y) = self.spawn_positions[self.selected_spawn];
                self.board.set_selected(x, y);
            }

            // backspace annule le choix de la case et  revient au menu des pièces.
            if e.is_key_pressed(KeyCode::Backspace) {
                self.choosing_spawn = false;
                self.spawn_positions.clear();
                self.selected_spawn = 0;
                self.board
                    .set_selected(self.previous_cursor.0, self.previous_cursor.1);
                return;
            }

            if e.is_key_pressed(KeyCode::Char(' ')) {
                if let Some(&(x, y)) = self.spawn_positions.get(self.selected_spawn) {
                    self.pending_request = serde_json::json!({
                        "type": "game",
                        "header": "spawn_piece",
                        "data": {
                            "piece": self.selected_piece,
                            "x": x,
                            "y": y,
                            "lobby_name": lobby_name
                        }
                    })
                    .to_string()
                        + "$";

                    // le serveur dit si c'est good ou pas donc on attend sa réponse avant de quitter le menu
                    self.choosing_spawn = false;
                    self.spawn_positions.clear();
                    self.board
                        .set_selected(self.previous_cursor.0, self.previous_cursor.1);
                    self.state = GameState::WaitEnd;
                }
            }

            return;
        }

        // choisir une piece que le joueur peut acheter
        if self.state == GameState::WaitChoosePiece {
            // 'P' ne rien acheter et terminer son tour
            if e.is_key_pressed(KeyCode::Char('p')) || e.is_key_pressed(KeyCode::Char('P')) {
                self.pending_request = serde_json::json!({
                    "type": "game",
                    "header": "skip_spawn",
                    "data": {
                        "lobby_name": lobby_name
                    }
                })
                .to_string()
                    + "$";
                self.state = GameState::WaitEnd;
                return;
            }

            let affordable = self.affordable_pieces();

            if affordable.is_empty() {
                return;
            }

            if !affordable.contains(&self.selected_piece) {
                self.selected_piece = affordable[0];
            }

            let current = affordable
                .iter()
                .position(|&id| id == self.selected_piece)
                .unwrap_or(0);

            if e.is_key_pressed(KeyCode::Right) || e.is_key_pressed(KeyCode::Down) {
                self.selected_piece = affordable[(current + 1) % affordable.len()];
            } else if e.is_key_pressed(KeyCode::Left) || e.is_key_pressed(KeyCode::Up) {
                self.selected_piece =
                    affordable[(current + affordable.len() - 1) % affordable.len()];
            }

            if e.is_key_pressed(KeyCode::Char(' ')) {
                let Some(cost) = self
                    .pieces
                    .get_piece_cost(self.selected_piece as i8, self.color as i8)
                else {
                    return;
                };

                if cost < 0 || self.players_points[self.color as usize] < cost as usize {
                    return;
                }

                self.previous_cursor = self.board.get_selected();

                self.spawn_positions = self.board.get_spawn_positions(self.color as i8);

                if self.spawn_positions.is_empty() {
                    return;
                }

                self.selected_spawn = 0;
                self.choosing_spawn = true;

                let (x, y) = self.spawn_positions[0];
                self.board.set_selected(x, y);
            }

            return;
        }

        // deplacement normal
        if e.is_key_pressed(KeyCode::Right) || e.is_key_pressed(KeyCode::Char('l')) {
            self.board.move_selected_area(1, 0);
        } else if e.is_key_pressed(KeyCode::Left) || e.is_key_pressed(KeyCode::Char('h')) {
            self.board.move_selected_area(-1, 0);
        } else if e.is_key_pressed(KeyCode::Down) || e.is_key_pressed(KeyCode::Char('j')) {
            self.board.move_selected_area(0, 1);
        } else if e.is_key_pressed(KeyCode::Up) || e.is_key_pressed(KeyCode::Char('k')) {
            self.board.move_selected_area(0, -1);
        } else if e.is_key_pressed(KeyCode::Char(' '))
            && self.state == GameState::WaitPlay
            && self.is_playing == self.color
        {
            if !self.board.is_selected_highlighted() {
                let (x, y) = self.board.select(&self.pieces, self.color as i8);

                if x != usize::MAX {
                    self.pending_request = serde_json::json!({
                        "type": "game",
                        "header": "piece_moves",
                        "data": {
                            "x": x,
                            "y": y,
                            "lobby_name": lobby_name
                        }
                    })
                    .to_string()
                        + "$";
                } else {
                    self.board.clear_highlights();
                }
            } else {
                let (ox, oy) = self.board.get_selected_piece();
                let (nx, ny) = self.board.get_selected();

                self.pending_request = serde_json::json!({
                    "type": "game",
                    "header": "move_piece",
                    "data": {
                        "ox": ox,
                        "oy": oy,
                        "nx": nx,
                        "ny": ny,
                        "lobby_name": lobby_name
                    }
                })
                .to_string()
                    + "$";

                self.awaiting_move = true;
                self.state = GameState::WaitEnd;
            }
        }
    }

    pub fn update(&self) {}

    pub fn display(&mut self, e: &mut ConsoleEngine) {
        // Afficher le plateau
        self.board.display(e, &self.pieces, &self.team_colors);

        // message d'attente
        if self.is_playing != self.color {
            let status_y = (self.board.board_size() * 3) as i32;
            e.print(0, status_y, "Un autre joueur est en train de jouer...");
        }

        // afficher le menu de sélection par-dessus le plateau
        if self.state == GameState::WaitChoosePiece && !self.choosing_spawn {
            self.display_all_pieces(e, self.color as i8, self.team_colors);
        }
        let offset_x = self.board.board_size() as i32 * 5 + 2;
        let offset_y = 1;
        // indication des commandes
        if self.choosing_spawn {
            e.fill_rect(offset_x, offset_y, 59, 4, pixel::pxl(' '));
            e.print(offset_x + 2, offset_y + 1, "Choisissez une case de spawn");
            e.print(
                offset_x + 2,
                offset_y + 2,
                "Gauche/Droite : changer de case",
            );
            e.print(
                offset_x + 2,
                offset_y + 3,
                "Espace : confirmer | Retour arriere : annuler",
            );
        }

        // informations des joueurs sous le plateau.
        self.display_players_info(e);
    }

    pub fn display_all_pieces(&mut self, e: &mut ConsoleEngine, team: i8, team_colors: [Color; 4]) {
        let offset_x = self.board.board_size() as i32 * 5 + 2;
        let offset_y = 1;
        let count = self.pieces.get_number_of_different_pieces() as usize;
        let points = self.players_points.get(team as usize).copied().unwrap_or(0);
        let affordable = self.affordable_pieces();

        // fond du menu
        e.fill_rect(offset_x, offset_y, 59, 10, pixel::pxl(' '));

        e.print(
            offset_x + 2,
            offset_y + 1,
            &format!("CHOIX DE LA PIECE A SPAWNER  (vos points : {})", points),
        );
        e.print(
            offset_x + 2,
            offset_y + 2,
            "Gauche/Droite : choisir | Espace : valider | P : passer",
        );

        if count == 0 {
            e.print(offset_x + 2, offset_y + 4, "Aucune piece disponible.");
            return;
        }

        if affordable.is_empty() {
            e.print(
                offset_x + 2,
                offset_y + 9,
                "Pas assez de points pour une piece : appuyez sur P pour passer.",
            );
        }

        let columns = 6;
        let spacing_x = 9;
        let spacing_y = 4;

        for i in 1..=count {
            let piece_id = i as i8 + team * count as i8;
            let piece = self.pieces.get_piece_from_id(piece_id);
            let cost = self.pieces.get_piece_cost(i as i8, team).unwrap_or(0);

            let x = offset_x + 2 + ((i - 1) % columns) as i32 * spacing_x;
            let y = offset_y + 5 + ((i - 1) / columns) as i32 * spacing_y;

            let can_buy = affordable.contains(&i);
            let selected = can_buy && i == self.selected_piece;

            // marqueur visuel pour la pièce actuellement sélectionnée.
            e.print(x, y - 1, if selected { ">" } else { " " });

            // nom de la pièce et son prix -> grisé si pas assez de points
            let label = format!("{} {} pts", piece.label, cost);
            if can_buy {
                e.print(x, y, &label);
            } else {
                e.print_fbg(x, y, &label, Color::DarkGrey, Color::Reset);
            }

            // soulignement colore
            let color = if selected {
                Color::White
            } else if can_buy {
                team_colors[team as usize]
            } else {
                Color::DarkGrey
            };

            e.line(x, y + 1, x + 5, y + 1, pixel::pxl_fg('-', color));

            if selected {
                e.print(x, y + 2, "Selectionnee");
            }
        }
    }

    fn display_players_info(&self, e: &mut ConsoleEngine) {
        let y = (self.board.board_size() * 3 + 1) as i32;

        e.print(0, y, "Joueurs :");

        let teams = (self.team_number.max(0) as usize).min(4);

        for player in 0..teams {
            let status = if player == self.is_playing as usize {
                " <-- joue"
            } else {
                ""
            };

            let line = format!(
                "Joueur {} : {} point(s){}",
                player, self.players_points[player], status
            );

            e.print(0, y + 1 + player as i32, &line);
        }
    }

    fn affordable_pieces(&self) -> Vec<usize> {
        // spectateur (couleur >= 4) ou partie non initialisée : rien à acheter.
        if self.color < 0 || self.color as usize >= self.players_points.len() {
            return Vec::new();
        }

        let count = self.pieces.get_number_of_different_pieces() as usize;

        (1..=count)
            .filter(|&id| {
                self.pieces
                    .get_piece_cost(id as i8, self.color as i8)
                    .map(|cost| {
                        cost >= 0 && self.players_points[self.color as usize] >= cost as usize
                    })
                    .unwrap_or(false)
            })
            .collect()
    }
}
