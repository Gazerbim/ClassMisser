use console_engine::{Color, ConsoleEngine, KeyCode, pixel};

use crate::{client::Client, types::protocol::Response};

pub struct Chat {
    buffer: String,
    send: bool,
    received: Vec<(usize, String)>,
    height: i32,
    length: i32,
    x: i32,
    y: i32,
}

impl Chat {
    pub fn new() -> Chat {
        Chat {
            buffer: String::with_capacity(50),
            send: false,
            received: Vec::new(),
            height: 30,
            length: 50,
            x: 80,
            y: 0,
        }
    }

    pub fn handling_events(&mut self, e: &ConsoleEngine, username_len: usize) {
        if self.buffer.len() + username_len + 2 < self.length as usize {
            let chars = [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ];
            let specials = [
                ' ', '.', ',', '?', '!', ')', '(', '-', '_', '+', '{', '}', '~', '#', '"', '|',
                '&', '%', '/', ':', '!', '*', '=', ']', '[', 'ç', '@', '1', '2', '3', '4', '5',
                '6', '7', '8', '9', '0', ';', '°', 'é',
            ];
            for c in chars {
                if e.is_key_pressed(KeyCode::Char(c)) || e.is_key_held(KeyCode::Char(c)) {
                    self.buffer.push(c);
                }
            }
            for s in specials {
                if e.is_key_pressed(KeyCode::Char(s)) || e.is_key_held(KeyCode::Char(s)) {
                    self.buffer.push(s);
                }
            }
        }
        if e.is_key_pressed(console_engine::KeyCode::Backspace) {
            self.buffer.pop();
        }
        if e.is_key_pressed(console_engine::KeyCode::Enter) {
            self.send = true;
        }
    }

    pub fn handle_server(&mut self, response: Response) {
        match response.header.as_str() {
            "new_message" => {
                match response.data["message"].as_str() {
                    Some(_m) => {
                        let sender = match response.data["sender"].as_i64() {
                            Some(_s) => _s as usize,
                            None => 5 as usize,
                        };
                        self.received.push((sender, _m.to_string().clone()));
                    }
                    None => {}
                };
            }
            _ => {}
        }
    }

    pub fn update(&mut self, client: &mut Client, lobby_name: &String) {
        if self.send {
            self.send = false;
            let message = serde_json::json!({
                "type": "chat",
                "header": "new_message",
                "data": {
                    "lobby_name": lobby_name,
                    "message": &self.buffer,
                }
            })
            .to_string()
                + "$";
            client.send(&message);
            self.buffer = String::with_capacity(self.length as usize);
        }
    }

    pub fn display(&self, e: &mut ConsoleEngine, colors: &[Color; 4]) {
        e.line(
            self.x,
            self.y,
            self.x,
            self.y + self.height,
            pixel::pxl('|'),
        );
        e.line(
            self.x + self.length,
            self.y,
            self.x + self.length,
            self.y + self.height,
            pixel::pxl('|'),
        );
        e.line(
            self.x,
            self.y,
            self.x + self.length,
            self.y,
            pixel::pxl('-'),
        );
        e.line(
            self.x,
            self.y + self.height,
            self.x + self.length,
            self.y + self.height,
            pixel::pxl('-'),
        );
        e.line(
            self.x,
            self.y + self.height - 4,
            self.x + self.length,
            self.y + self.height - 4,
            pixel::pxl('-'),
        );
        e.set_pxl(self.x, self.y + self.height, pixel::pxl('+'));
        e.set_pxl(self.x + self.length, self.y + self.height, pixel::pxl('+'));
        e.set_pxl(self.x, self.y, pixel::pxl('+'));
        e.set_pxl(self.x + self.length, self.y, pixel::pxl('+'));
        e.set_pxl(self.x, self.y + self.height - 4, pixel::pxl('+'));
        e.set_pxl(
            self.x + self.length,
            self.y + self.height - 4,
            pixel::pxl('+'),
        );
        e.print(self.x + 1, self.y + self.height - 2, &self.buffer);
        for i in 0..self.received.len() {
            let color = if self.received[i].0 < 4 {
                colors[self.received[i].0]
            } else {
                Color::Reset
            };
            e.print_fbg(
                self.x + 1,
                (i + 1) as i32 + self.y,
                &self.received[i].1,
                color,
                Color::Reset,
            );
        }
    }
}
