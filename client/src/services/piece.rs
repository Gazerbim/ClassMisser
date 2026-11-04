use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Piece {
    name: String,
    pub index: i8,
    cost: i8,
    pub label: char,
    pub label_bis: char,
    pub team: i8,
    is_king: bool,
}

impl Piece {
    pub fn new(
        name: String,
        index: i8,
        cost: i8,
        label: char,
        label_bis: char,
        team: i8,
        is_king: bool,
    ) -> Piece {
        Piece {
            name: name,
            index: index,
            cost: cost,
            label: label,
            label_bis: label_bis,
            team: team,
            is_king: is_king,
        }
    }

    pub fn get_cost(&self) -> i8 {
        self.cost
    }

    pub fn get_label(&self) -> &char {
        &self.label
    }

    pub fn is_king(&self) -> bool {
        self.is_king
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
