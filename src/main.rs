mod control;
mod lang;
mod model;
mod view;

use crate::control::{update, Message, DIFFICULTY_STORAGE, HELPER_STORAGE, LANGUAGE_STORAGE};
use crate::model::Model;
use crate::view::view;
use seed::browser::web_storage::LocalStorage;
use seed::prelude::*;

fn init(_url: Url, _orders: &mut impl Orders<Message>) -> Model {
    let difficulty = LocalStorage::get(DIFFICULTY_STORAGE).unwrap_or_default();
    let language = LocalStorage::get(LANGUAGE_STORAGE).unwrap_or_default();
    let helper = LocalStorage::get(HELPER_STORAGE).unwrap_or_default();
    Model::new(difficulty, helper, language)
}

fn main() {
    seed::App::start("app", init, update, view);
}
