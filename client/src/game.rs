use console_engine::ConsoleEngine;
use serde_json::Value;

use crate::services::board::Board;
use crate::services::piece::Piece;
use crate::services::pieces::Pieces;
use crate::types::protocol::Response;
use crate::types::states::GameState;

pub struct Game {
    color: i32,
    board: Board,
    pieces: Pieces,
    state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            color: -1,
            board: Board::new(),
            pieces: Pieces::new(),
            state: GameState::WaitInit,
        }
    }

    pub fn set_color(&mut self, new_color: i32) {
        self.color = new_color;
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
        self.board.init(board_size, team_number, xc, yc, all_pieces);
        self.pieces.load_pieces(team_number);
        self.color = color;
    }

    pub fn handle_server(&mut self, response: Response) {}

    pub fn handling_hevents(&self) {}
    pub fn update(&self) {}
    pub fn display(&mut self, e: &mut ConsoleEngine) {
        e.print(10, 10, &"La partie est lancée !");
        self.board.display(e, &self.pieces);
    }
}
