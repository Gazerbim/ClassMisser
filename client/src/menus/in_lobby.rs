pub struct InLobby {
    name: String,
}

impl InLobby {
    pub fn new(name: String) -> InLobby {
        InLobby { name: name }
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name.clone();
    }

    pub fn handling_events(&self) {}

    pub fn update(&self) {}

    pub fn display(&self, e: &mut console_engine::ConsoleEngine, top_left: (i32, i32)) {
        let mut text = "Dans le salon nommé ".to_string();
        text.push_str(&self.name);
        text.push_str(" !");
        e.print(top_left.0, top_left.1, &text)
    }
}
