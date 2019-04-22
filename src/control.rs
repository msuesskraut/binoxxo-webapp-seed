use crate::model::{Difficulty, Model};
use binoxxo::field::Field;
use seed::prelude::*;

pub const DIFFICULTY_STORAGE: &str = "Binoxxo-Difficulty";
pub const LANGUAGE_STORAGE: &str = "Binoxxo-Language";

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

fn toggle_field(model: &mut Model, pos: &CellPos) {
    let field = next_field(model.board.get(pos.col, pos.row));
    if field == Field::Empty {
        model.board.clear(pos.col, pos.row);
    } else {
        model.board.set(pos.col, pos.row, field);
    }
}

fn new_game(model: &mut Model, difficulty: Difficulty) {
    seed::log!(model.board.to_string());
    let storage = seed::storage::get_storage();
    if let Some(storage) = storage {
        seed::log!(format!("Store {} = {}", DIFFICULTY_STORAGE, difficulty));
        seed::storage::store_data(&storage, DIFFICULTY_STORAGE, &difficulty);
    }
    let new_model = Model::new(difficulty, model.language);
    model.board = new_model.board;
    model.difficulty = new_model.difficulty;
    model.editable = new_model.editable;
}

fn clear_board(model: &mut Model) {
    let size = model.get_size();
    for col in 0..size {
        for row in 0..size {
            if model.editable.is_editable(col, row) && Field::Empty != model.board.get(col, row) {
                model.board.clear(col, row);
            }
        }
    }
}

fn change_language(model: &mut Model) {
    model.language = model.language.next();
    let storage = seed::storage::get_storage();
    if let Some(storage) = storage {
        seed::log!(format!(
            "Store {} = {}",
            LANGUAGE_STORAGE,
            model.language.to_string()
        ));
        seed::storage::store_data(&storage, LANGUAGE_STORAGE, &model.language);
    }
}

pub fn update(message: Message, model: &mut Model) -> Update<Message> {
    seed::log!(format!("Got {:?}", message));

    match message {
        Message::Toggle(pos) => toggle_field(model, &pos),
        Message::NewGame(difficulty) => new_game(model, difficulty),
        Message::Clear => clear_board(model),
        Message::ToggleLanguage => change_language(model),
    }
    Render.into()
}
