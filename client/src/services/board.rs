use crate::services::move_json::Move;
use crate::services::pieces::Pieces;
use array2d::Array2D;
use console_engine::{Color, ConsoleEngine, pixel};

pub struct Board {
    data: Array2D<i8>,
    selected: (usize, usize),
    selected_piece: (usize, usize),
    to_highlight: Vec<Move>,
    board_size: usize,
    team_number: i8,
}

impl Board {
    pub fn new() -> Board {
        //let grid: Array2D<i8> = Array2D::filled_with(0, board_size, board_size);
        let grid: Array2D<i8> = Array2D::filled_with(0, 1, 1);
        Board {
            data: grid,
            selected: (usize::MAX, usize::MAX),
            selected_piece: (usize::MAX, usize::MAX),
            to_highlight: Vec::new(),
            board_size: 1,
            team_number: 4,
        }
    }

    pub fn init(&mut self, board_size: usize, team_number: i8) {
        self.team_number = team_number;
        self.board_size = board_size;
        self.data = Array2D::filled_with(0, board_size, board_size);
    }

    fn draw_square(
        &self,
        engine: &mut ConsoleEngine,
        x: usize,
        y: usize,
        value: i8,
        highlight: bool,
        highlight_playable: bool,
        pieces: &Pieces,
    ) {
        const WIDTH: usize = 5;
        const HEIGHT: usize = 3;

        let screen_x = x * WIDTH;
        let screen_y = y * HEIGHT;

        let background = if highlight == true {
            Color::Rgb {
                r: 230,
                g: 20,
                b: 20,
            }
        } else if highlight_playable == true {
            Color::Rgb {
                r: 230,
                g: 230,
                b: 20,
            }
        } else if value == -1 {
            Color::Rgb { r: 0, g: 0, b: 0 }
        } else if (x + y) % 2 == 0 {
            Color::Rgb {
                r: 230,
                g: 230,
                b: 230,
            }
        } else {
            Color::Rgb {
                r: 100,
                g: 60,
                b: 30,
            }
        };

        // Fond de la case
        for dy in 0..HEIGHT {
            for dx in 0..WIDTH {
                engine.set_pxl(
                    (screen_x + dx) as i32,
                    (screen_y + dy) as i32,
                    pixel::pxl_bg(' ', background),
                );
            }
        }

        // Pièce
        if value > 0 {
            engine.print((screen_x + 2) as i32, (screen_y + 1) as i32, "");
        }
    }

    pub fn display(&mut self, engine: &mut ConsoleEngine, pieces: &Pieces) {
        for y in 0..self.board_size {
            for x in 0..self.board_size {
                self.draw_square(
                    engine,
                    x,
                    y,
                    self.get(x, y),
                    self.selected == (x, y),
                    self.to_highlight
                        .iter()
                        .any(|m| m.x == x as i8 && m.y == y as i8),
                    pieces,
                );
            }
        }
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
