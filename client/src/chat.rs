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
    username: String,
    chat_length: usize,
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
            username: "".to_string(),
            chat_length: 0,
        }
    }

    pub fn set_username(&mut self, username: &String) {
        self.username = username.clone();
        self.buffer = self.username.clone();
        self.buffer.push_str(": ");
        self.chat_length += username.len() + 2;
    }

    pub fn handling_events(&mut self, e: &ConsoleEngine) {
        if self.buffer.len() + self.username.len() - 1 < 3 * self.length as usize - 2 {
            let chars = [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ];
            let cap = [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ];
            let specials = [
                ' ', '.', ',', '?', '!', ')', '(', '-', '_', '+', '{', '}', '~', '#', '"', '|',
                '&', '%', '/', ':', '*', '=', ']', '[', 'ç', '@', '1', '2', '3', '4', '5', '6',
                '7', '8', '9', '0', ';', '°', 'é', '\'',
            ];
            for c in chars {
                if e.is_key_pressed(KeyCode::Char(c)) || e.is_key_held(KeyCode::Char(c)) {
                    self.buffer.push(c);
                }
            }
            for c in cap {
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
        if e.is_key_pressed(console_engine::KeyCode::Backspace)
            && self.buffer.len() > self.username.len() + 2
        {
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
                        self.receive_message(_m.to_string().clone(), sender);
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
            self.buffer = self.username.clone();
            self.buffer.push_str(": ");
        }
    }

    pub fn display(&self, e: &mut ConsoleEngine, colors: &[Color; 4], focus: bool) {
        let focus_color = if focus { Color::Green } else { Color::Reset };

        e.line(
            self.x,
            self.y,
            self.x,
            self.y + self.height - 4,
            pixel::pxl('|'),
        );
        e.line(
            self.x,
            self.y + self.height - 4,
            self.x,
            self.y + self.height,
            pixel::pxl_fg('|', focus_color),
        );
        e.line(
            self.x + self.length,
            self.y,
            self.x + self.length,
            self.y + self.height - 4,
            pixel::pxl('|'),
        );
        e.line(
            self.x + self.length,
            self.y + self.height - 4,
            self.x + self.length,
            self.y + self.height,
            pixel::pxl_fg('|', focus_color),
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
            pixel::pxl_fg('-', focus_color),
        );
        e.line(
            self.x,
            self.y + self.height - 4,
            self.x + self.length,
            self.y + self.height - 4,
            pixel::pxl_fg('-', focus_color),
        );

        e.set_pxl(
            self.x,
            self.y + self.height,
            pixel::pxl_fg('+', focus_color),
        );
        e.set_pxl(
            self.x + self.length,
            self.y + self.height,
            pixel::pxl_fg('+', focus_color),
        );
        e.set_pxl(self.x, self.y, pixel::pxl('+'));
        e.set_pxl(self.x + self.length, self.y, pixel::pxl('+'));
        e.set_pxl(
            self.x,
            self.y + self.height - 4,
            pixel::pxl_fg('+', focus_color),
        );
        e.set_pxl(
            self.x + self.length,
            self.y + self.height - 4,
            pixel::pxl_fg('+', focus_color),
        );

        //e.print(self.x + 1, self.y + self.height - 3, &self.buffer);
        let cuts = self.get_message_cuts(&self.buffer);
        for j in 0..cuts.len() {
            e.print(self.x + 1, (j as i32) + self.y + self.height - 3, &cuts[j]);
        }

        let mut decalage = 0;
        for i in 0..self.received.len() {
            let color = if self.received[i].0 < 4 {
                colors[self.received[i].0]
            } else {
                Color::Reset
            };
            //diviser les messages en x lignes
            let cuts = self.get_message_cuts(&self.received[i].1);
            for j in 0..cuts.len() {
                e.print_fbg(
                    self.x + 1,
                    1 + self.y + decalage,
                    &cuts[j],
                    color,
                    Color::Reset,
                );
                decalage += 1;
            }
        }
    }

    fn get_message_cuts(&self, message: &String) -> Vec<String> {
        let ulenght = (self.length - 1) as usize;
        if message.len() as i32 > 2 * self.length - 2 {
            return vec![
                message[0..(ulenght)].to_string(),
                message[(ulenght)..(ulenght * 2)].to_string(),
                message[(ulenght * 2)..message.len()].to_string(),
            ];
        } else if message.len() as i32 > self.length - 1 {
            return vec![
                message[0..ulenght].to_string(),
                message[ulenght..message.len()].to_string(),
            ];
        }
        return vec![message[0..message.len()].to_string()];
    }

    fn receive_message(&mut self, message: String, sender: usize) {
        let lines = self.get_message_cuts(&message).len();
        self.received.push((sender, message));
        self.chat_length += lines;
        if self.chat_length as i32 > self.height - 1 {
            let gain = self.get_message_cuts(&self.received[0].1).len();
            self.received.remove(0);
            self.chat_length -= gain;
        }
    }
}
