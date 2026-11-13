use console_engine::{Color, ConsoleEngine, pixel};

pub struct Popup {
    text: String,
    button1: String,
    button2: String,
    state: bool,
    selection: usize,
    enter_pressed: bool,
}

impl Popup {
    pub fn new(text: String, button1: String, button2: String) -> Popup {
        Popup {
            text: text,
            button1: button1,
            button2: button2,
            state: false,
            selection: 0,
            enter_pressed: false,
        }
    }

    pub fn switch_state(&mut self) {
        self.state = !self.state;
        self.enter_pressed = false;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn get_selection(&self) -> usize {
        self.selection
    }

    pub fn get_enter_pressed(&self) -> bool {
        self.enter_pressed
    }

    pub fn handling_events(&mut self, e: &ConsoleEngine) {
        if e.is_key_pressed(console_engine::KeyCode::Right)
            || e.is_key_pressed(console_engine::KeyCode::Char('l'))
            || e.is_key_pressed(console_engine::KeyCode::Char('k'))
        {
            if self.selection == 0 {
                self.selection = 1;
            }
        }
        if e.is_key_pressed(console_engine::KeyCode::Left)
            || e.is_key_pressed(console_engine::KeyCode::Char('h'))
            || e.is_key_pressed(console_engine::KeyCode::Char('j'))
        {
            if self.selection == 1 {
                self.selection = 0;
            }
        }
        if e.is_key_pressed(console_engine::KeyCode::Enter) {
            self.enter_pressed = true;
        }
    }

    pub fn update(&self) {}

    pub fn diplay(&self, e: &mut ConsoleEngine, top_left: (i32, i32)) {
        let length = if self.text.len() + 4 > self.button1.len() + self.button2.len() + 9 {
            self.text.len() as i32 + 4
        } else {
            (self.button1.len() + self.button2.len() + 7) as i32
        };
        let height: i32 = 8;
        e.fill_rect(
            top_left.0,
            top_left.1,
            top_left.0 + length,
            top_left.1 + height,
            pixel::pxl_bg(' ', console_engine::Color::Reset),
        );
        e.line(
            top_left.0,
            top_left.1,
            top_left.0 + length,
            top_left.1,
            pixel::pxl('#'),
        );
        e.line(
            top_left.0,
            top_left.1 + height,
            top_left.0 + length,
            top_left.1 + height,
            pixel::pxl('#'),
        );
        e.line(
            top_left.0 + length,
            top_left.1,
            top_left.0 + length,
            top_left.1 + height,
            pixel::pxl('#'),
        );
        e.line(
            top_left.0,
            top_left.1,
            top_left.0,
            top_left.1 + height,
            pixel::pxl('#'),
        );

        e.print(top_left.0 + 2, top_left.1 + 2, &self.text);
        e.print(top_left.0 + 3, top_left.1 + 5, &self.button1);
        e.print(
            top_left.0 + 3 + self.button1.len() as i32 + 4,
            top_left.1 + 5,
            &self.button2,
        );
        let color1 = [Color::Blue, Color::Reset][self.selection];
        let color2 = [Color::Reset, Color::Blue][self.selection];
        self.little_square(
            e,
            (top_left.0 + 2, top_left.1 + 4),
            self.button1.len() as i32,
            color1,
        );
        self.little_square(
            e,
            (top_left.0 + 6 + self.button1.len() as i32, top_left.1 + 4),
            self.button2.len() as i32,
            color2,
        );
    }

    fn little_square(
        &self,
        e: &mut ConsoleEngine,
        top_left: (i32, i32),
        length: i32,
        color: console_engine::Color,
    ) {
        let height = 2;
        let length = length + 2;
        e.line(
            top_left.0,
            top_left.1,
            top_left.0 + length,
            top_left.1,
            pixel::pxl_fg('-', color),
        );
        e.line(
            top_left.0,
            top_left.1 + height,
            top_left.0 + length,
            top_left.1 + height,
            pixel::pxl_fg('-', color),
        );
        e.line(
            top_left.0 + length,
            top_left.1,
            top_left.0 + length,
            top_left.1 + height,
            pixel::pxl_fg('|', color),
        );
        e.line(
            top_left.0,
            top_left.1,
            top_left.0,
            top_left.1 + height,
            pixel::pxl_fg('|', color),
        );
        e.set_pxl(top_left.0, top_left.1, pixel::pxl_fg('+', color));
        e.set_pxl(top_left.0 + length, top_left.1, pixel::pxl_fg('+', color));
        e.set_pxl(
            top_left.0 + length,
            top_left.1 + 2,
            pixel::pxl_fg('+', color),
        );
        e.set_pxl(top_left.0, top_left.1 + 2, pixel::pxl_fg('+', color));
    }
}
