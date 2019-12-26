mod control;
mod lang;
mod model;
mod view;

use crate::control::{update, Message, DIFFICULTY_STORAGE, HELPER_STORAGE, LANGUAGE_STORAGE};
use crate::model::{Difficulty, Helper, Language, Model};
use crate::view::view;
use seed::prelude::*;

fn load_difficulty() -> Option<Difficulty> {
    if let Some(storage) = seed::storage::get_storage() {
        if let Ok(Some(loaded_serialized)) = storage.get_item(DIFFICULTY_STORAGE) {
            return serde_json::from_str(&loaded_serialized).ok();
        }
    }
    None
}

fn load_language() -> Option<Language> {
    if let Some(storage) = seed::storage::get_storage() {
        if let Ok(Some(loaded_serialized)) = storage.get_item(LANGUAGE_STORAGE) {
            return serde_json::from_str(&loaded_serialized).ok();
        }
    }
    None
}

fn load_helper() -> Option<Helper> {
    if let Some(storage) = seed::storage::get_storage() {
        if let Ok(Some(loaded_serialized)) = storage.get_item(HELPER_STORAGE) {
            return serde_json::from_str(&loaded_serialized).ok();
        }
    }
    None
}

fn after_mount(_url: Url, _orders: &mut impl Orders<Message>) -> AfterMount<Model> {
    let difficulty = load_difficulty().unwrap_or_default();
    let language = load_language().unwrap_or_default();
    let helper = load_helper().unwrap_or_default();
    let model = Model::new(difficulty, helper, language);
    AfterMount::new(model)
}

#[wasm_bindgen]
pub fn render() {
    seed::App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
