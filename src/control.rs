use crate::model::{Difficulty, Model};
use binoxxo::field::Field;

pub const DIFFICULTY_STORAGE: &str = "Binoxxo-Difficulty";

#[derive(Clone, Copy, Debug)]
pub struct CellPos {
    pub col: usize,
    pub row: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    NewGame(Difficulty),
    Toggle(CellPos),
    Clear,
    ToggleLanguage,
}

fn next_field(field: Field) -> Field {
    match field {
        Field::Empty => Field::X,
        Field::X => Field::O,
        Field::O => Field::Empty,
    }
}

fn toggle_field(model: Model, pos: &CellPos) -> Model {
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
    let storage = seed::storage::get_storage();
    if let Some(storage) = storage {
        seed::log!(format!("Store {} = {}", DIFFICULTY_STORAGE, difficulty));
        seed::storage::store_data(&storage, DIFFICULTY_STORAGE, &difficulty);
    }
    model
}

fn clear_board(model: Model) -> Model {
    let mut model = model;
    let size = model.get_size();
    for col in 0..size {
        for row in 0..size {
            if model.editable.is_editable(col, row) && Field::Empty != model.board.get(col, row) {
                model.board.clear(col, row);
            }
        }
    }
    model
}

pub fn update(message: Message, model: Model) -> Model {
    seed::log!(format!("Got {:?}", message));

    match message {
        Message::Toggle(pos) => toggle_field(model, &pos),
        Message::NewGame(difficulty) => new_game(difficulty),
        Message::Clear => clear_board(model),
        Message::ToggleLanguage => model,
    }
}
