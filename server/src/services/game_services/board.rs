use crate::services::game_services::move_json::Move;
use crate::services::game_services::pieces::Pieces;
use array2d::Array2D;

pub struct Board {
    data: Array2D<i8>,
    selected: (usize, usize),
    selected_piece: (usize, usize),
    to_highlight: Vec<Move>,
    board_size: usize,
    team_number: i8,
}

impl Board {
    pub fn new(team_number: i8, board_size: usize) -> Board {
        let grid: Array2D<i8> = Array2D::filled_with(0, board_size, board_size);
        Board {
            data: grid,
            selected: (usize::MAX, usize::MAX),
            selected_piece: (usize::MAX, usize::MAX),
            to_highlight: Vec::new(),
            board_size: board_size,
            team_number: team_number,
        }
    }

    pub fn init(&mut self, board_size: usize, team_number: i8) {
        self.team_number = team_number;
        self.board_size = board_size;
        self.data = Array2D::filled_with(0, board_size, board_size);
    }

    fn clear_highlights(&mut self) {
        self.to_highlight.clear();
    }

    fn set(&mut self, x: usize, y: usize, value: i8) {
        self.data[(y, x)] = value;
    }

    fn get(&self, x: usize, y: usize) -> i8 {
        return self.data[(y, x)];
    }

    fn move_piece(&mut self, x_from: usize, y_from: usize, x_to: usize, y_to: usize) -> i8 {
        let what_is_to = self.get(x_to, y_to);
        self.set(x_to, y_to, self.get(x_from, y_from));
        self.set(x_from, y_from, 0);

        return what_is_to;
    }

    fn move_selected_area(&mut self, dx: i8, dy: i8) {
        let tx = self.selected.0 as i16 + dx as i16;
        let ty = self.selected.1 as i16 + dy as i16;

        if tx >= 0 && tx < self.board_size as i16 && ty >= 0 && ty < self.board_size as i16 {
            self.selected = (tx as usize, ty as usize);
        }
    }

    fn select(&mut self, pieces: &Pieces) {
        let piece = self.get(self.selected.0, self.selected.1);
        if piece >= 1 {
            self.selected_piece = self.selected.clone();
            // let moves: Vec<Move> =
            //     pieces.get_moves_from_position(self.selected.0, self.selected.1, piece, self);
            // self.to_highlight = moves;
            self.selected_piece = self.selected.clone();
        }
    }

    fn is_highlighted(&self, x: usize, y: usize) -> bool {
        return self
            .to_highlight
            .iter()
            .any(|m| m.x == x as i8 && m.y == y as i8);
    }

    fn is_selected_highlighted(&self) -> bool {
        return self.is_highlighted(self.selected.0, self.selected.1);
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        return self.get(x, y) > 0;
    }

    fn get_selected_piece_id(&self) -> i8 {
        return self.get(self.selected_piece.0, self.selected_piece.1);
    }
}
