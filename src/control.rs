use binoxxo::field::Field;
use crate::model::{Difficulty, Model};

#[derive(Clone, Debug)]
pub struct CellPos {
    pub col: usize,
    pub row: usize
}

#[derive(Clone, Debug)]
pub enum Message {
    NewGame(Difficulty),
    Toggle(CellPos)
}

fn next_field(field: Field) -> Field {
    match field {
        Field::Empty => Field::X,
        Field::X => Field::O,
        Field::O => Field::Empty
    }
}

fn toggle_field(model: Model, pos: CellPos) -> Model {
    let mut model = model; 
    let field = next_field(model.board.get(pos.col, pos.row));
    if field == Field::Empty {
        model.board.clear(pos.col, pos.row);
    } else {
        model.board.set(pos.col, pos.row, field);
    }
    model
}

fn new_game(difficulty: Difficulty) -> Model {
    let model = Model::new(difficulty);
    seed::log!(model.board.to_string());
    model
}

pub fn update(message: Message, model: Model) -> Model {
    seed::log!(format!("Got {:?}", message));

    match message {
        Message::Toggle(pos) => toggle_field(model, pos),
        Message::NewGame(difficulty) => new_game(difficulty)
    }
}