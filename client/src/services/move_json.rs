use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Move {
    pub x: i8,
    pub y: i8,
}
