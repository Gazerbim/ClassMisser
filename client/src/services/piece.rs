use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Piece {
    name: String,
    pub index: i8,
    cost: i8,
    pub label: String,
    pub custom_label: String,
    pub team: i8,
    is_king: bool,
}

impl Piece {
    pub fn new(
        name: String,
        index: i8,
        cost: i8,
        label: String,
        custom_label: String,
        team: i8,
        is_king: bool,
    ) -> Piece {
        Piece {
            name: name,
            index: index,
            cost: cost,
            label: label,
            custom_label: custom_label,
            team: team,
            is_king: is_king,
        }
    }

    pub fn get_cost(&self) -> i8 {
        self.cost
    }

    pub fn get_label(&self, selected_texture: usize) -> &String {
        if selected_texture == 0 {
            &self.label
        } else {
            &self.custom_label
        }
    }
    

    pub fn is_king(&self) -> bool {
        self.is_king
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
