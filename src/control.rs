use binoxxo::field::{Field, Board};
use crate::model::Model;

#[derive(Clone, Debug)]
pub struct CellPos {
    pub col: usize,
    pub row: usize
}

#[derive(Clone, Debug)]
pub enum Message {
    Toggle(CellPos)
}

fn next_field(field: Field) -> Field {
    match field {
        Field::Empty => Field::X,
        Field::X => Field::O,
        Field::O => Field::Empty
    }
}

fn toggle_field(board: &mut Board, pos: CellPos) {
    let field = next_field(board.get(pos.col, pos.row));
    if field == Field::Empty {
        board.clear(pos.col, pos.row);
    } else {
        board.set(pos.col, pos.row, field);
    }
}

pub fn update(message: Message, model: Model) -> Model {
    let mut model = model;
    match message {
        Message::Toggle(pos) => toggle_field(&mut model.board, pos)
    }
    model
}