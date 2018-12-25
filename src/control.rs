use crate::model::Model;

#[derive(Clone, Debug)]
pub enum Message {}

pub fn update(_message: Message, model: Model) -> Model {
    model
}