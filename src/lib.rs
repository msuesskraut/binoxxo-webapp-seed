mod control;
mod model;
mod view;

use crate::control::{update, DIFFICULTY_STORAGE};
use crate::model::{Difficulty, Model};
use crate::view::view;
use wasm_bindgen::prelude::*;

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
