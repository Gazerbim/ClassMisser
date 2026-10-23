use crate::services::move_json::Move;
use crate::services::pieces::Pieces;
use array2d::Array2D;
use console_engine::{Color, ConsoleEngine, pixel};

pub struct Board {
    data: Array2D<i8>,
    selected: (usize, usize),
    selected_piece: (usize, usize),
    to_highlight: Vec<Move>,
    pub board_size: usize,
    team_number: i8,
}

impl Board {

    pub fn board_size(&self) -> usize {
        self.board_size
    }

    pub fn set_selected(&mut self, x: usize, y: usize) {
        if x < self.board_size && y < self.board_size {
            self.selected = (x, y);
        }
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.get(x, y) == 0
    }

        
    pub fn get_spawn_positions(
        &self,
        player: i8,
    ) -> Vec<(usize, usize)> {
        let n = self.board_size;

        if n == 0 || player < 0 || player >= self.team_number {
            return Vec::new();
        }

        // Identifiants des rois définis par le serveur.
        let king_id = match player {
            0 => -5,
            1 => -4,
            2 => -3,
            3 => -2,
            _ => return Vec::new(),
        };

        // Recherche de la position actuelle du roi.
        let mut king_position = None;

        for y in 0..n {
            for x in 0..n {
                if self.get(x, y) == king_id {
                    king_position = Some((x, y));
                    break;
                }
            }

            if king_position.is_some() {
                break;
            }
        }

        let Some((kx, ky)) = king_position else {
            return Vec::new();
        };

        let mut positions = Vec::new();

        for dy in -1_i32..=1 {
            for dx in -1_i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let x = kx as i32 + dx;
                let y = ky as i32 + dy;

                if x < 0 || y < 0 || x >= n as i32 || y >= n as i32 {
                    continue;
                }

                let position = (x as usize, y as usize);

                if self.is_empty(position.0, position.1) {
                    positions.push(position);
                }
            }
        }

        positions
    }


    pub fn new() -> Board {
        //let grid: Array2D<i8> = Array2D::filled_with(0, board_size, board_size);
        let grid: Array2D<i8> = Array2D::filled_with(0, 1, 1);
        Board {
            data: grid,
            selected: (0, 0),
            selected_piece: (usize::MAX, usize::MAX),
            to_highlight: Vec::new(),
            board_size: 1,
            team_number: 4,
        }
    }

    pub fn init(
        &mut self,
        board_size: usize,
        team_number: i8,
        x: Vec<usize>,
        y: Vec<usize>,
        pieces: Vec<i8>,
    ) {
        self.team_number = team_number;
        self.board_size = board_size;
        self.data = Array2D::filled_with(0, board_size, board_size);
        for i in 0..x.len() {
            self.data[(y[i], x[i])] = pieces[i];
        }
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
        team_colors: &[Color; 4],
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
            Color::Rgb {
                r: 75,
                g: 75,
                b: 75,
            }
        } else if value < -1 && value >= -5 {
            team_colors[(value + 5) as usize] //couleurs des cases dans les coins pour les équipes
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
            let color: Color = team_colors[pieces.get_team(value) as usize];
            engine.set_pxl(
                (screen_x + 2) as i32,
                (screen_y + 1) as i32,
                pixel::pxl_bg(' ', color),
            );
            engine.set_pxl(
                (screen_x + 1) as i32,
                (screen_y + 1) as i32,
                pixel::pxl_fbg(
                    *pieces.get_piece_from_id(value).get_label(),
                    Color::Black,
                    color,
                ),
            );
        }
    }

    pub fn display(
        &mut self,
        engine: &mut ConsoleEngine,
        pieces: &Pieces,
        team_colors: &[Color; 4],
    ) {
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
                    team_colors,
                );
            }
        }
    }

    pub fn clear_highlights(&mut self) {
        self.to_highlight.clear();
    }

    pub fn highlight(&mut self, x: &Vec<usize>, y: &Vec<usize>) {
        for i in 0..x.len() {
            self.to_highlight.push(Move {
                x: x[i] as i8,
                y: y[i] as i8,
            });
        }
    }

    fn set(&mut self, x: usize, y: usize, value: i8) {
        self.data[(y, x)] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        return self.data[(y, x)];
    }

    pub fn get_selected(&self) -> (usize, usize) {
        self.selected
    }

    fn move_piece(&mut self, x_from: usize, y_from: usize, x_to: usize, y_to: usize) -> i8 {
        let what_is_to = self.get(x_to, y_to);
        self.set(x_to, y_to, self.get(x_from, y_from));
        self.set(x_from, y_from, 0);

        return what_is_to;
    }

    pub fn move_selected_area(&mut self, dx: i8, dy: i8) {
        let tx = self.selected.0 as i16 + dx as i16;
        let ty = self.selected.1 as i16 + dy as i16;

        if tx >= 0 && tx < self.board_size as i16 && ty >= 0 && ty < self.board_size as i16 {
            self.selected = (tx as usize, ty as usize);
        }
    }

    pub fn update(&mut self, x: Vec<usize>, y: Vec<usize>, p: Vec<i8>) {
        for i in 0..x.len() {
            self.set(x[i], y[i], p[i]);
        }
        self.clear_highlights();
    }

    pub fn select(&mut self, pieces: &Pieces, player: i8) -> (usize, usize) {
        let mut piece = self.get(self.selected.0, self.selected.1);
        piece -= pieces.get_number_different_pieces() * player;
        if piece >= 1 && piece <= pieces.get_number_different_pieces() {
            self.selected_piece = self.selected.clone();
        } else {
            self.selected_piece = (usize::MAX, usize::MAX);
        }
        self.selected_piece
    }

    pub fn get_selected_piece(&self) -> (usize, usize) {
        self.selected_piece
    }

    pub fn to_highlight_is_empty(&self) -> bool {
        self.to_highlight.is_empty()
    }

    fn is_highlighted(&self, x: usize, y: usize) -> bool {
        return self
            .to_highlight
            .iter()
            .any(|m| m.x == x as i8 && m.y == y as i8);
    }

    pub fn is_selected_highlighted(&self) -> bool {
        return self.is_highlighted(self.selected.0, self.selected.1);
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        return self.get(x, y) > 0;
    }

    fn get_selected_piece_id(&self) -> i8 {
        return self.get(self.selected_piece.0, self.selected_piece.1);
    }
}
