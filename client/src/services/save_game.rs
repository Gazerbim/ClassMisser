use array2d::Array2D;
use serde::{Serialize, Deserialize};

use crate::services::board::Board;
use crate::services::pieces::Pieces;
use console_engine::{Color, ConsoleEngine, pixel};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseBoard{
    pub data: Vec<Vec<i8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame{
    pieces: Pieces,
    boards: Vec<BaseBoard>,
}

impl SaveGame {
    pub fn new(pieces: Pieces) -> SaveGame {
        SaveGame {
            pieces: pieces,
            boards: Vec::new(),
        }
    }

    pub fn add_board(&mut self, board: &BaseBoard) {
        let base_board = BaseBoard {
            data: board.data.clone(),
        };
        self.boards.push(base_board);
    }

    pub fn get_board(&self, index: usize) -> Option<&BaseBoard> {
        self.boards.get(index)
    }

    pub fn get_boards(&self) -> &Vec<BaseBoard> {
        &self.boards
    }

    /// sauve garde de la partie
    pub fn save_boards_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dir) = std::path::Path::new(file_path).parent() {
            std::fs::create_dir_all(dir)?; // si le dossier n'existe pas -> creer
        }
        let serialized = serde_json::to_string(self)?;
        std::fs::write(file_path, serialized)?;
        Ok(())
    }

    pub fn from_file(file_path: &str) -> Result<SaveGame, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(file_path)?;
        if let Ok(save) = serde_json::from_str::<SaveGame>(&data) {
            return Ok(save);
        }
        let boards: Vec<BaseBoard> = serde_json::from_str(&data)?;
        let mut pieces = Pieces::new();
        pieces.load_pieces(4); // deux types de formats : soit avec les pieces, soit sans
        Ok(SaveGame { pieces, boards })
    }

    pub fn len(&self) -> usize {
        self.boards.len()
    }

    pub fn load_boards_from_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(file_path)?;
        let boards: Vec<BaseBoard> = serde_json::from_str(&data)?;
        self.boards = boards;
        Ok(())
    }

    pub fn add_board_from_board(&mut self, board: &Board) {
        let base_board = BaseBoard {
            data: board.get_board().rows_iter().map(|row| row.cloned().collect()).collect(),
        };
        self.boards.push(base_board);
    }

    pub fn get_board_as_board(&self, index: usize) -> Option<Board> {
        self.boards.get(index).map(|base_board| {
            let grid: Array2D<i8> = Array2D::from_rows(&base_board.data).unwrap();
            let mut board = Board::new();
            board.set_board(grid);
            return board
        })
    }

    fn draw_square(
        &self,
        engine: &mut ConsoleEngine,
        x: usize,
        y: usize,
        value: i8,
        team_colors: &[Color; 4],
        selected_texture: usize,
    ) {
        const WIDTH: usize = 5;
        const HEIGHT: usize = 3;

        let screen_x = x * WIDTH;
        let screen_y = y * HEIGHT;

        let background = if value == -1 {
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

        //fond de la case
        for dy in 0..HEIGHT {
            for dx in 0..WIDTH {
                engine.set_pxl(
                    (screen_x + dx) as i32,
                    (screen_y + dy) as i32,
                    pixel::pxl_bg(' ', background),
                );
            }
        }

        // pièce
        if value > 0 {
            let color: Color = team_colors[self.pieces.get_team(value) as usize];
            engine.set_pxl(
                (screen_x + 2) as i32,
                (screen_y + 1) as i32,
                pixel::pxl_bg(' ', color),
            );
            engine.set_pxl(
                (screen_x + 1) as i32,
                (screen_y + 1) as i32,
                pixel::pxl_fbg(
                    *self.pieces.get_piece_from_id(value).get_label(selected_texture),
                    Color::Black,
                    color,
                ),
            );
        }
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        if let Some(board) = self.boards.last() {
            if y < board.data.len() && x < board.data[y].len() {
                return board.data[y][x];
            }
        }
        -1
    }

    pub fn display(
        &self,
        engine: &mut ConsoleEngine,
        team_colors: &[Color; 4],
        selected_texture: usize,
        frame_index: usize,
    ) {
        let Some(board) = self.boards.get(frame_index) else {
            return;
        };
        for y in 0..board.data.len() {
            for x in 0..board.data[y].len() {
                self.draw_square(engine, x, y, board.data[y][x], team_colors, selected_texture);
            }
        }
    }

}
