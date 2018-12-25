use binoxxo::field::Field;
use binoxxo::rules::{is_board_full, is_board_valid};
use crate::model::*;
use crate::control::{CellPos, Message};
use seed::prelude::*;

macro_rules! table {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Table);
            $ (
                    $part.update(&mut el);
            )*
            el
        }
    };
}

macro_rules! tr {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Tr);
            $ (
                    $part.update(&mut el);
            )*
            el
        }
    };
}

macro_rules! td {
    ( $($part:expr),* $(,)* ) => {
        {
            let mut el = El::empty(seed::dom_types::Tag::Td);
            $ (
                    $part.update(&mut el);
            )*
            el
        }
    };
}

fn view_field(field: Field) -> El<Message> {
    use seed::*;

    let classes = match field {
        Field::Empty => "far fa-square",
        Field::X => "fas fa-times",
        Field::O => "far fa-circle"
    };

    i![attrs!{"class" => classes}]
}

fn view_cell(model: &Model, col: usize, row: usize) -> El<Message> {
    use seed::*;

    let field = model.board.get(col, row);
    let editable = model.editable.is_editable(col, row);
    let class = if editable { "guess" } else { "" };

    td![
        attrs!{"class" => class.to_string() },
        view_field(field),
        simple_ev("click", Message::Toggle(CellPos{col, row}))
    ]
}

fn view_row(model: &Model, row: usize) -> El<Message> {
    let size = model.get_size();
    let cells: Vec<El<Message>> = (0..size)
        .map(|col| view_cell(
                model,
                col,
                row))
        .collect();
    tr![cells]
} 

fn view_board(model: &Model) -> El<Message> {
    use seed::*;

    let size = model.get_size();
    let rows: Vec<El<Message>> = (0..size)
        .map(|row| view_row(model, row))
        .collect();
    div![
        attrs!{"id" => "board"},
        if is_board_full(&model.board) {
            let valid = is_board_valid(&model.board);
            let text = if valid {
                "Success!"
            } else {
                "Sorry. Try again."
            };

            div![
                attrs!{"class" => if valid { "alert alert-success" } else { "alert alert-danger" }},
                text
            ]
        } else {
            seed::empty()
        },
        table![
            rows
        ]
    ]
}

pub fn view(model: Model) -> El<Message> {
    view_board(&model)
}