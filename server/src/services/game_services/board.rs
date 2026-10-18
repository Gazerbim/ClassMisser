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
    pub fn new(team_number: i8, board_size: usize, pieces: &Pieces) -> Board {
        let mut grid: Array2D<i8> = Array2D::filled_with(0, board_size, board_size);
        //initialisation du tableau en supposant board_size = 9
        grid[(0, 0)] = -5;
        grid[(board_size - 1, board_size - 1)] = -4;
        if team_number >= 3 {
            grid[(board_size - 1, 0)] = -3;
        }
        if team_number >= 4 {
            grid[(0, board_size - 1)] = -2;
        }
        grid[(1, 1)] = 1;
        grid[(board_size - 2, board_size - 2)] = 6;
        if team_number >= 3 {
            grid[(board_size - 2, 1)] = 11;
        }
        if team_number >= 4 {
            grid[(1, board_size - 2)] = 16;
        }
        grid[(4, 4)] = -1;
        grid[(0, 4)] = -1;
        grid[(4, 0)] = -1;
        grid[(8, 4)] = -1;
        grid[(4, 8)] = -1;
        Board {
            data: grid,
            selected: (usize::MAX, usize::MAX),
            selected_piece: (usize::MAX, usize::MAX),
            to_highlight: Vec::new(),
            board_size: board_size,
            team_number: team_number,
        }
    }

    pub fn new_empty() -> Board {
        Board {
            data: Array2D::filled_with(0, 1, 1),
            selected: (usize::MAX, usize::MAX),
            selected_piece: (usize::MAX, usize::MAX),
            to_highlight: Vec::new(),
            board_size: 0,
            team_number: 0,
        }
    }

    pub fn init(&mut self, board_size: usize, team_number: i8) {
        self.team_number = team_number;
        self.board_size = board_size;
        self.data = Array2D::filled_with(0, board_size, board_size);
    }

    pub fn get_simplfied_board(&self) -> (Vec<usize>, Vec<usize>, Vec<i8>) {
        let mut x: Vec<usize> = Vec::new();
        let mut y: Vec<usize> = Vec::new();
        let mut p: Vec<i8> = Vec::new();
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                if self.data[(i, j)] != 0 {
                    x.push(j);
                    y.push(i);
                    p.push(self.data[(i, j)]);
                }
            }
        }
        (x, y, p)
    }

    pub fn get_board_size(&self) -> usize {
        self.board_size
    }

    pub fn set_selected_piece(&mut self, x: usize, y: usize) {
        self.selected_piece = (x, y);
    }

    fn clear_highlights(&mut self) {
        self.to_highlight.clear();
    }

    fn set(&mut self, x: usize, y: usize, value: i8) {
        self.data[(y, x)] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        return self.data[(y, x)];
    }

    pub fn move_piece(&mut self, x_from: usize, y_from: usize, x_to: usize, y_to: usize) -> i8 {
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

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        return self.get(x, y) > 0;
    }

    pub fn get_selected_piece_id(&self) -> i8 {
        return self.get(self.selected_piece.0, self.selected_piece.1);
    }
}
