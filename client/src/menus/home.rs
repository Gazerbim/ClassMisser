use console_engine::{Color, ConsoleEngine, pixel};

pub struct Home {
    colors: Vec<Color>,
    text: Vec<Vec<bool>>,
    starting_color: usize,
    slower: usize,
}

impl Home {
    pub fn new() -> Home {
        Home {
            starting_color: 0,
            slower: 0,
            colors: vec![
                Color::Red,
                Color::Red,
                Color::DarkYellow,
                Color::Yellow,
                Color::Yellow,
                Color::Green,
                Color::Green,
                Color::Cyan,
                Color::Cyan,
                Color::Blue,
                Color::DarkBlue,
            ],
            text: vec![
                vec![
                    false, false, true, true, true, true, true, true, true, true, false, false,
                ],
                vec![
                    false, false, true, true, true, true, true, true, true, true, false, false,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![
                    true, true, false, false, false, false, false, false, false, false, true, true,
                ],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![
                    true, true, true, true, true, true, true, true, true, true, true, true,
                ],
                vec![
                    true, true, true, true, true, true, true, true, true, true, true, true,
                ],
                vec![false, false, true, true],
                vec![false, false, true, true],
                vec![false, false, false, false, true, true, true],
                vec![false, false, false, false, true, true, true],
                vec![false, false, true, true],
                vec![false, false, true, true],
                vec![
                    true, true, true, true, true, true, true, true, true, true, true, true,
                ],
                vec![
                    true, true, true, true, true, true, true, true, true, true, true, true,
                ],
            ],
        }
    }

    pub fn update(&mut self) {
        self.slower += 1;
        if self.slower >= 2 {
            self.starting_color += 1;
            self.slower = 0;
            if self.starting_color == self.colors.len() {
                self.starting_color = 0;
            }
        }
    }

    pub fn display(&self, e: &mut ConsoleEngine, top_left: (i32, i32)) {
        for i in 0..self.text.len() {
            for j in 0..self.text[i].len() {
                let mut color_id = (i + j) as i32 - self.starting_color as i32;
                while color_id >= self.colors.len() as i32 {
                    color_id -= self.colors.len() as i32;
                }
                while color_id < 0 {
                    color_id += self.colors.len() as i32;
                }
                if self.text[i][j] {
                    e.set_pxl(
                        top_left.0 + i as i32,
                        top_left.1 + j as i32,
                        pixel::pxl_fg('#', self.colors[color_id as usize]),
                    );
                }
            }
        }
    }
}
