use console_engine::{KeyCode, pixel};

pub struct TextInput {
    title: String,
    buffer: String,
    enter_pressed: bool,
}

impl TextInput {
    pub fn new(title: String, capacity: usize) -> TextInput {
        TextInput {
            title: title,
            buffer: String::with_capacity(capacity),
            enter_pressed: false,
        }
    }

    pub fn handling_events(&mut self, e: &console_engine::ConsoleEngine) {
        let chars = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        for c in chars {
            if e.is_key_pressed(KeyCode::Char(c)) && self.buffer.len() < self.buffer.capacity() {
                self.buffer.push(c);
            }
        }
        if e.is_key_pressed(console_engine::KeyCode::Backspace) {
            self.buffer.pop();
        }
        if e.is_key_pressed(KeyCode::Enter) && self.buffer.len() > 0 {
            self.enter_pressed = true;
        }
    }

    pub fn update(&self) -> String {
        if self.enter_pressed {
            return self.buffer.clone();
        } else {
            return String::from("");
        }
    }

    pub fn display(&mut self, e: &mut console_engine::ConsoleEngine, top_left: (i32, i32)) {
        e.print(
            top_left.0 + 2 + (self.buffer.capacity() as i32 / 2) - (self.title.len() as i32 / 2),
            top_left.1 - 2,
            &self.title,
        );
        e.line(
            top_left.0,
            top_left.1,
            top_left.0 + (self.buffer.capacity() as i32) + 4,
            top_left.1,
            pixel::pxl('#'),
        );
        e.line(
            top_left.0,
            top_left.1 + 4,
            top_left.0 + (self.buffer.capacity() as i32) + 4,
            top_left.1 + 4,
            pixel::pxl('#'),
        );
        e.line(
            top_left.0,
            top_left.1,
            top_left.0,
            top_left.1 + 4,
            pixel::pxl('#'),
        );
        e.line(
            top_left.0 + (self.buffer.capacity() as i32) + 4,
            top_left.1,
            top_left.0 + (self.buffer.capacity() as i32) + 4,
            top_left.1 + 4,
            pixel::pxl('#'),
        );
        e.print(top_left.0 + 2, top_left.1 + 2, &self.buffer);
    }
}
