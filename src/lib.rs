mod control;
mod model;
mod view;

use wasm_bindgen::prelude::*;
use crate::model::Model;
use crate::control::update;
use crate::view::view;

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);
}