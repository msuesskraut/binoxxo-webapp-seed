use crate::model::{Difficulty, Model};
use binoxxo::field::Field;
use seed::prelude::*;
use seed::browser::web_storage::LocalStorage;

pub const DIFFICULTY_STORAGE: &str = "Binoxxo-Difficulty";
pub const LANGUAGE_STORAGE: &str = "Binoxxo-Language";
pub const HELPER_STORAGE: &str = "Binoxxo-Helper";

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
    ToggleHelper,
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
    seed::log!(format!("Store {} = {}", DIFFICULTY_STORAGE, difficulty));
    LocalStorage::insert(DIFFICULTY_STORAGE, &difficulty).ok();

    let new_model = Model::new(difficulty, model.helper, model.language);
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
    seed::log!(format!(
        "Store {} = {}",
        LANGUAGE_STORAGE,
        model.language.to_string()
    ));
    LocalStorage::insert(LANGUAGE_STORAGE, &model.language).ok();
}

fn change_helper(model: &mut Model) {
    model.helper = model.helper.toggle();
    seed::log!(format!(
        "Store {} = {}",
        HELPER_STORAGE,
        model.helper.to_string()
    ));
    LocalStorage::insert(HELPER_STORAGE, &model.helper).ok();
}

pub fn update(message: Message, model: &mut Model, _: &mut impl Orders<Message>) {
    seed::log!(format!("Got {:?}", message));

    match message {
        Message::Toggle(pos) => toggle_field(model, &pos),
        Message::NewGame(difficulty) => new_game(model, difficulty),
        Message::Clear => clear_board(model),
        Message::ToggleLanguage => change_language(model),
        Message::ToggleHelper => change_helper(model),
    }
}
