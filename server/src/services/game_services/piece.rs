use crate::services::game_services::move_json::Move;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Piece {
    name: String,
    pub index: i8,
    pub moves: Vec<Move>,
    pub extendable: bool,
    cost: i8,
    label: String,
    can_eat_with_basic_move: bool,
    pub team: i8,
    eating_moves: Vec<Move>,
    is_king: bool,
}

impl Piece {
    fn new(
        name: String,
        index: i8,
        moves: Vec<Move>,
        extendable: bool,
        cost: i8,
        label: String,
        can_eat_with_basic_move: bool,
        team: i8,
        eating_moves: Vec<Move>,
        is_king: bool,
    ) -> Piece {
        Piece {
            name: name,
            index: index,
            moves: moves,
            extendable: extendable,
            cost: cost,
            label: label,
            can_eat_with_basic_move: can_eat_with_basic_move,
            team: team,
            eating_moves: eating_moves,
            is_king: is_king,
        }
    }

    pub fn get_cost(&self) -> i8 {
        self.cost
    }

    pub fn can_eat_with_basic_move(&self) -> bool {
        self.can_eat_with_basic_move
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_eating_moves(&self) -> &Vec<Move> {
        &self.eating_moves
    }

    pub fn is_king(&self) -> bool {
        self.is_king
    }
}
