use console_engine::ConsoleEngine;

use crate::services::board::Board;
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

    pub fn init_game(&mut self, color: i32, board_size: usize, team_number: i8) {
        self.board.init(board_size, team_number);
        self.pieces.load_pieces(team_number);
        self.color = color;
    }

    pub fn handle_server(&mut self, response: Response) {
        match response.header.as_str() {
            "setup" => {}
            _ => {}
        }
    }

    pub fn handling_hevents(&self) {}
    pub fn update(&self) {}
    pub fn display(&mut self, e: &mut ConsoleEngine) {
        e.print(10, 10, &"La partie est lancée !");
        self.board.display(e, &self.pieces);
    }
}
