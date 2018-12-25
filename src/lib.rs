mod control;
mod model;
mod view;

use wasm_bindgen::prelude::*;
use crate::model::{Model, Difficulty};
use crate::control::{update, DIFFICULTY_STORAGE};
use crate::view::view;

fn load_difficulty() -> Option<Difficulty> {
    if let Some(storage) = seed::storage::get_storage() {
        if let Ok(Some(loaded_serialized)) = storage.get_item(DIFFICULTY_STORAGE) {
            return serde_json::from_str(&loaded_serialized).ok();
        }
    }
    None
}

#[wasm_bindgen]
pub fn render() {
    let difficulty = load_difficulty().unwrap_or_default();
    seed::run(Model::new(difficulty), update, view, "main", None);
}