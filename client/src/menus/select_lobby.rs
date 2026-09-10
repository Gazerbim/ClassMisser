pub struct SelectLobby {
    selector: i32,
    elements: Vec<String>,
}

impl SelectLobby {
    pub fn new(v: Vec<String>) -> SelectLobby {
        SelectLobby {
            selector: 0,
            elements: v,
        }
    }

    pub fn handling_events(&mut self, e: &console_engine::ConsoleEngine) -> String {
        if e.is_key_pressed(console_engine::KeyCode::Up) && self.selector > 0 {
            self.selector = self.selector - 1;
        }
        if e.is_key_pressed(console_engine::KeyCode::Down)
            && self.selector < (self.elements.len() as i32) - 1
        {
            self.selector = self.selector + 1;
        }
        if e.is_key_pressed(console_engine::KeyCode::Enter) {
            return self.elements[self.selector as usize].clone();
        } else {
            return String::new();
        }
    }

    pub fn update(&self) {}

    pub fn display(&self, e: &mut console_engine::ConsoleEngine, top_left: (i32, i32)) {
        for i in 0..(self.elements.len() as i32) {
            let mut first: String = " ".to_owned();
            if i == self.selector {
                first = ">".to_owned();
            }
            first.push_str(&self.elements[i as usize]);
            e.print(top_left.0, top_left.1 + i, &first)
        }
    }
}
