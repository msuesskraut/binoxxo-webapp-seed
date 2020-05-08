mod control;
mod lang;
mod model;
mod view;

use crate::control::{update, Message, DIFFICULTY_STORAGE, HELPER_STORAGE, LANGUAGE_STORAGE};
use crate::model::Model;
use crate::view::view;
use seed::prelude::*;
use seed::browser::web_storage::LocalStorage;

fn after_mount(_url: Url, _orders: &mut impl Orders<Message>) -> AfterMount<Model> {
    let difficulty = LocalStorage::get(DIFFICULTY_STORAGE).unwrap_or_default();
    let language = LocalStorage::get(LANGUAGE_STORAGE).unwrap_or_default();
    let helper = LocalStorage::get(HELPER_STORAGE).unwrap_or_default();
    let model = Model::new(difficulty, helper, language);
    AfterMount::new(model)
}

#[wasm_bindgen]
pub fn render() {
    seed::App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
