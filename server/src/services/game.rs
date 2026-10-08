use std::{fs};

use array2d::Array2D;
use serde::{Deserialize, Serialize};


const TEAM_NUMBER:i8 = 4;
const BOARD_SIZE:usize = 8;

#[derive(Debug, Deserialize, Clone)]
struct Move {
    x: i8,
    y: i8,
}

#[derive(Debug, Deserialize, Clone)]
struct Piece{
    name:String,
    index: i8,
    moves: Vec<Move>,
    extendable: bool,
    cost: i8,
    label: String,
    team: i8,
    eating_moves: Vec<Move>,
}

impl Piece{
    fn new(name:String, index: i8, moves: Vec<Move>, extendable: bool, cost: i8, label: String, team: i8, eating_moves: Vec<Move>)->Piece{
        Piece {name:name, index:index, moves:moves, extendable:extendable, cost:cost, label:label, team:team, eating_moves:eating_moves}
    }
}


#[derive(Debug, Clone)]
struct Pieces {
    pieces: Vec<Piece>,
}

impl Pieces {
    fn load_pieces() -> Pieces {
        let content = fs::read_to_string("src/pieces.json")
            .expect("error when reading the file");

        let pieces: Vec<Piece> = serde_json::from_str(&content)
            .expect("error when parsing json");

        let mut pieces_final: Vec<Piece> = Vec::new();


        for i in 0..TEAM_NUMBER{
            for piece in &pieces{
                let mut npiece = piece.clone();
                npiece.team = i;
                npiece.index = piece.index + TEAM_NUMBER * i;
                pieces_final.push(npiece);
            }
        }

        Pieces { pieces: pieces_final }
    }

    fn get_piece_from_id(&self, id: i8) -> &Piece {
        self.pieces.iter().find(|piece| piece.index == id).expect("the id isnt in pieces")
    }

    fn get_team(&self, id: i8)-> i8{
        return self.get_piece_from_id(id).team;
    }

    fn get_moves_from_position(&self, x: usize, y: usize, id: i8, board: &Board) -> Vec<Move> {
        let piece = self.get_piece_from_id(id);
        let mut moves = Vec::new();

        for m in &piece.moves {
            let mut i: i8 = 0;
            let mut new_x = m.x + x as i8;
            let mut new_y = m.y + y as i8;

            while new_x >= 0 && new_x < BOARD_SIZE as i8 && new_y >= 0 && new_y < BOARD_SIZE as i8{
                
                
                if (board.get(new_x as usize, new_y as usize) == -1)|| (board.is_occupied(new_x as usize, new_y as usize) && self.get_team(board.get(new_x as usize, new_y as usize)) == self.get_team(board.get(x,y))){
                    break;
                }

                moves.push(Move {
                    x: new_x,
                    y: new_y,
                });

                if piece.extendable == false || board.is_occupied(new_x as usize, new_y as usize){
                    break;
                }

                i += 1;
                new_x = m.x*i + x as i8;
                new_y = m.y*i + y as i8;
            }
        }

        for m in &piece.eating_moves{
            let new_x = m.x + x as i8;
            let new_y = m.y + y as i8;

            if new_x >= 0 && new_x < BOARD_SIZE as i8 && new_y >= 0 && new_y < BOARD_SIZE as i8{
                if board.is_occupied(new_x as usize, new_y as usize) && self.get_team(board.get(new_x as usize, new_y as usize)) != self.get_team(board.get(x, y)){
                    moves.push(Move {
                        x: new_x,
                        y: new_y,
                    });
                }
            }
        }

        return moves;
    }
}




fn serialize_array<S>(
    array: &Array2D<i8>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let rows: Vec<Vec<i8>> = (0..array.num_rows())
        .map(|r| {
            (0..array.num_columns())
                .map(|c| *array.get(r, c).unwrap())
                .collect()
        })
        .collect();

    rows.serialize(serializer)
}


#[derive(Serialize)]
struct Board{
    #[serde(serialize_with = "serialize_array")]
    data: Array2D<i8>,
}


impl Board{
    fn new()->Board{
        let grid:Array2D<i8> = Array2D::filled_with(0,BOARD_SIZE, BOARD_SIZE);
        Board{data:grid}
    }


    fn set(&mut self, x:usize, y:usize, value: i8){
        self.data[(y,x)] = value;
    }

    fn get(&self, x:usize, y:usize)->i8{
        return self.data[(y,x)];
    }


    //move piece from one point to another
    fn move_piece(&mut self, x_from:usize, y_from:usize, x_to:usize, y_to:usize)->i8{
        let what_is_to = self.get(x_to, y_to);
        self.set(x_to, y_to, self.get(x_from, y_from));
        self.set(x_from, y_from, 0);

        return what_is_to;
    }

    //returns wether the cell is occupied by a piece
    fn is_occupied(&self, x:usize,y:usize) -> bool{
        return self.get(x, y) > 0;
    }

}