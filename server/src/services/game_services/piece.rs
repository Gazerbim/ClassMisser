use crate::services::game_services::move_json::Move;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Piece {
    name: String,
    pub index: i8,
    pub moves: Vec<Move>,
    extendable: bool,
    cost: i8,
    label: String,
    pub team: i8,
    eating_moves: Vec<Move>,
}

impl Piece {
    fn new(
        name: String,
        index: i8,
        moves: Vec<Move>,
        extendable: bool,
        cost: i8,
        label: String,
        team: i8,
        eating_moves: Vec<Move>,
    ) -> Piece {
        Piece {
            name: name,
            index: index,
            moves: moves,
            extendable: extendable,
            cost: cost,
            label: label,
            team: team,
            eating_moves: eating_moves,
        }
    }
}
