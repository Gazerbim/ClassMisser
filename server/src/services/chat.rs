use serde_json::Value;

pub struct Chat {}

impl Chat {
    pub fn new() -> Chat {
        Chat {}
    }

    pub fn handle_request(&self, sender: usize, header: &str, data: &Value) -> String {
        match data["message"].as_str() {
            Some(_m) => {
                serde_json::json!({
                    "type": "chat",
                    "header": "new_message",
                    "data": {
                        "message": _m,
                        "sender": sender
                    }
                })
                .to_string()
                    + "$"
            }
            None => "".to_string(),
        }
    }
}
