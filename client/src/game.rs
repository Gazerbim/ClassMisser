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
        if self.color == self.is_playing {
            self.state = GameState::WaitPlay;
        }
    }

    pub fn handle_server(&mut self, response: Response) {
        match response.header.as_str() {
            "piece_moves" => {
                let x: Option<Vec<usize>> = response.data["x"]
                    .as_array()
                    .map(|arr| arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as usize))
                        .collect());

                let y: Option<Vec<usize>> = response.data["y"]
                    .as_array()
                    .map(|arr| arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as usize))
                        .collect());

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

                if let Some(player) = response.data["player_playing"].as_i64() {
                    if (0..=3).contains(&player) {
                        self.is_playing = player as i32;

                        // IMPORTANT : ne pas écraser l'état du menu
                        // lorsqu'on reçoit l'update après un déplacement.
                        if self.state != GameState::WaitChoosePiece
                            && !self.choosing_spawn
                        {
                            self.state = if self.is_playing == self.color {
                                GameState::WaitPlay
                            } else {
                                GameState::WaitEnd
                            };
                        }
                    }
                }

                if let Some(points) = response.data["players_points"].as_array() {
                    for (i, point) in points.iter().enumerate().take(4) {
                        if let Some(value) = point.as_u64() {
                            self.players_points[i] = value as usize;
                        }
                    }
                }
            }
            "spawn_rejected" => {
                self.choosing_spawn = false;
                self.spawn_positions.clear();
                self.selected_spawn = 0;
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

    
    
    pub fn handling_hevents(
        &mut self,
        e: &mut ConsoleEngine,
        lobby_name: String,
    ) {
        // Phase 1 : choisir une case de spawn.
        if self.choosing_spawn {
            if self.spawn_positions.is_empty() {
                self.choosing_spawn = false;
                self.state = GameState::WaitChoosePiece;
                return;
            }

            let previous = self.selected_spawn;

            if e.is_key_pressed(KeyCode::Right)
                || e.is_key_pressed(KeyCode::Down)
            {
                self.selected_spawn =
                    (self.selected_spawn + 1) % self.spawn_positions.len();
            } else if e.is_key_pressed(KeyCode::Left)
                || e.is_key_pressed(KeyCode::Up)
            {
                self.selected_spawn =
                    (self.selected_spawn + self.spawn_positions.len() - 1)
                        % self.spawn_positions.len();
            }

            if previous != self.selected_spawn {
                let (x, y) = self.spawn_positions[self.selected_spawn];
                self.board.set_selected(x, y);
            }

            if e.is_key_pressed(KeyCode::Char(' ')) {
                if let Some(&(x, y)) =
                    self.spawn_positions.get(self.selected_spawn)
                {
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
                    .to_string() + "$";

                    // Le serveur confirmera le spawn dans update_board.
                    self.choosing_spawn = false;
                    self.spawn_positions.clear();
                    self.state = GameState::WaitEnd;
                }
            }

            return;
        }

        // Phase 2 : choisir une pièce que le joueur peut acheter.
        if self.state == GameState::WaitChoosePiece {
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

            if e.is_key_pressed(KeyCode::Right)
                || e.is_key_pressed(KeyCode::Down)
            {
                self.selected_piece =
                    affordable[(current + 1) % affordable.len()];
            } else if e.is_key_pressed(KeyCode::Left)
                || e.is_key_pressed(KeyCode::Up)
            {
                self.selected_piece = affordable[
                    (current + affordable.len() - 1) % affordable.len()
                ];
            }

            if e.is_key_pressed(KeyCode::Char(' ')) {
                let Some(cost) = self.pieces.get_piece_cost(
                    self.selected_piece as i8,
                    self.color as i8,
                ) else {
                    return;
                };

                if cost < 0
                    || self.players_points[self.color as usize] < cost as usize
                {
                    return;
                }

                self.previous_cursor = self.board.get_selected();

                self.spawn_positions =
                    self.board.get_spawn_positions(self.color as i8);

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

        // Phase 3 : déplacement normal.
        if e.is_key_pressed(KeyCode::Right)
            || e.is_key_pressed(KeyCode::Char('l'))
        {
            self.board.move_selected_area(1, 0);
        } else if e.is_key_pressed(KeyCode::Left)
            || e.is_key_pressed(KeyCode::Char('h'))
        {
            self.board.move_selected_area(-1, 0);
        } else if e.is_key_pressed(KeyCode::Down)
            || e.is_key_pressed(KeyCode::Char('j'))
        {
            self.board.move_selected_area(0, 1);
        } else if e.is_key_pressed(KeyCode::Up)
            || e.is_key_pressed(KeyCode::Char('k'))
        {
            self.board.move_selected_area(0, -1);
        } else if e.is_key_pressed(KeyCode::Char(' '))
            && self.state == GameState::WaitPlay
            && self.is_playing == self.color
        {
            if !self.board.is_selected_highlighted() {
                let (x, y) =
                    self.board.select(&self.pieces, self.color as i8);

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
                    .to_string() + "$";
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
                .to_string() + "$";

                self.selected_piece = 0;
                self.state = GameState::WaitChoosePiece;
            }
        }
    }




    pub fn update(&self) {}

    
    pub fn display(&mut self, e: &mut ConsoleEngine) {
        // Afficher le plateau une seule fois.
        self.board.display(e, &self.pieces, &self.team_colors);

        if self.is_playing != self.color {
            e.print(4, 30, "Un autre joueur est en train de jouer...");
        }

        // Afficher le menu de sélection par-dessus le plateau.
        if self.state == GameState::WaitChoosePiece {
            self.display_all_pieces(
                e,
                self.color as i8,
                self.team_colors,
            );
        }

        // Pendant le choix de la case de spawn, indiquer les commandes.
        if self.choosing_spawn {
            e.print(2, 1, "Choisissez une case de spawn");
            e.print(2, 2, "Gauche/Droite : changer de case");
            e.print(2, 3, "Espace : confirmer");
        }

        // Informations des joueurs sous le plateau.
        self.display_players_info(e);
    }


    
    pub fn display_all_pieces(
        &mut self,
        e: &mut ConsoleEngine,
        team: i8,
        team_colors: [Color; 4],
    ) {
        let count = self.pieces.get_number_of_different_pieces() as usize;

        e.print(2, 1, "CHOIX DE LA PIECE A SPAWNER");
        e.print(2, 2, "Gauche/Droite : choisir | Espace : continuer");

        if count == 0 {
            e.print(2, 4, "Aucune piece disponible.");
            return;
        }

        // Vérifier que la sélection est valide.
        if self.selected_piece == 0 || self.selected_piece > count {
            self.selected_piece = 1;
        }

        let columns = 6;
        let spacing_x = 9;
        let spacing_y = 4;

        for i in 1..=count {
            let piece_id =
                i as i8 + team * self.pieces.get_number_of_different_pieces() as i8;

            let piece = self.pieces.get_piece_from_id(piece_id);

            let x = 2 + ((i - 1) % columns) as i32 * spacing_x;
            let y = 5 + ((i - 1) / columns) as i32 * spacing_y;

            let selected = i == self.selected_piece;

            // Marqueur visuel pour la pièce actuellement sélectionnée.
            if selected {
                e.print(x, y - 1, ">");
            } else {
                e.print(x, y - 1, " ");
            }

            // Nom de la pièce.
            e.print(x, y, &format!("{}", piece.label));

            // Soulignement coloré.
            let color = if selected {
                Color::White
            } else {
                team_colors[team as usize]
            };

            e.line(
                x,
                y + 1,
                x + 5,
                y + 1,
                pixel::pxl_fg('-', color),
            );

            if selected {
                e.print(x, y + 2, "Selectionnee");
            }
        }

        e.print(
            2,
            24,
            &format!("Piece choisie : {}/{}", self.selected_piece, count),
        );
    }


    fn display_players_info(&self, e: &mut ConsoleEngine) {
        let y = (self.board.board_size() * 3 + 1) as i32;

        e.print(0, y, "Joueurs :");

        for player in 0..4 {
            let status = if player == self.is_playing as usize {
                " <-- joue"
            } else {
                ""
            };

            let line = format!(
                "Joueur {} : {} point(s){}",
                player,
                self.players_points[player],
                status
            );

            e.print(0, y + 1 + player as i32, &line);
        }
    }

    fn affordable_pieces(&self) -> Vec<usize> {
        let count = self.pieces.get_number_of_different_pieces() as usize;

        (1..=count)
            .filter(|&id| {
                self.pieces
                    .get_piece_cost(id as i8, self.color as i8)
                    .map(|cost| {
                        cost >= 0
                            && self.players_points[self.color as usize]
                                >= cost as usize
                    })
                    .unwrap_or(false)
            })
            .collect()
    }
}
