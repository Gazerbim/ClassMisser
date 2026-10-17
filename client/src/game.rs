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
    pending_request: String,
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
            pending_request: "".to_string(),
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
    }

    pub fn handle_server(&mut self, response: Response) {
        match response.header.as_str() {
            "piece_moves" => {
                let mut x: Vec<usize> = Vec::new();
                let mut y: Vec<usize> = Vec::new();
                let mut broken = false;
                match response.data["x"].as_array() {
                    Some(_x) => {
                        for v in _x {
                            match v.as_i64() {
                                None => broken = true,
                                Some(_v) => {
                                    x.push(_v as usize);
                                }
                            }
                        }
                    }
                    None => {}
                };
                match response.data["y"].as_array() {
                    Some(_y) => {
                        for v in _y {
                            match v.as_i64() {
                                None => broken = true,
                                Some(_v) => {
                                    y.push(_v as usize);
                                }
                            }
                        }
                    }
                    None => {}
                };
                if y.len() != 0 && x.len() == y.len() && !broken {
                    self.board.clear_highlights();
                    self.board.highlight(&x, &y);
                }
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

    pub fn handling_hevents(&mut self, e: &ConsoleEngine, lobby_name: String) {
        if self.is_playing == self.color {
            //c'est à mon tour de jouer
            if e.is_key_pressed(KeyCode::Right) || e.is_key_pressed(KeyCode::Char('l')) {
                self.board.move_selected_area(1, 0);
            } else if e.is_key_pressed(KeyCode::Left) || e.is_key_pressed(KeyCode::Char('h')) {
                self.board.move_selected_area(-1, 0);
            } else if e.is_key_pressed(KeyCode::Down) || e.is_key_pressed(KeyCode::Char('j')) {
                self.board.move_selected_area(0, 1);
            } else if e.is_key_pressed(KeyCode::Up) || e.is_key_pressed(KeyCode::Char('k')) {
                self.board.move_selected_area(0, -1);
            } else if e.is_key_pressed(KeyCode::Char(' ')) {
                if self.board.to_highlight_is_empty() {
                    self.board.select(&self.pieces);
                    let (x, y) = &self.board.get_selected();
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
                        + "$"
                } else {
                    if self.board.is_selected_highlighted() {
                        //faire requête pour bouger pièce
                    }
                }
            }
        }
    }
    pub fn update(&self) {}
    pub fn display(&mut self, e: &mut ConsoleEngine) {
        self.board.display(e, &self.pieces, &self.team_colors);
        if self.is_playing != self.color {
            e.line(
                2,
                3,
                50,
                50,
                pixel::pxl_bg(' ', self.team_colors[self.is_playing as usize]),
            );
            e.print(4, 30, "est en train de jouer...");
        }
    }
}
