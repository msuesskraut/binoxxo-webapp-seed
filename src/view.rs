use crate::control::{CellPos, Message};
use crate::model::*;
use binoxxo::field::Field;
use binoxxo::rules::{is_board_full, is_board_valid};
use fluent_bundle::{FluentBundle, FluentValue};
use seed::prelude::*;

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

struct ViewBuilder<'a> {
    bundle: FluentBundle<'a>,
    model: &'a Model,
}

impl<'a> ViewBuilder<'a> {
    fn tr(&self, id: &str) -> String {
        self.bundle
            .format(id, None)
            .expect(&format!("tr({}) failed", id))
            .0
    }

    fn view_field(&self, field: Field) -> El<Message> {
        use seed::*;

        let classes = match field {
            Field::Empty => "fas fa-circle",
            Field::X => "fas fa-times",
            Field::O => "far fa-circle",
        };

        let mut i = i![attrs! {"class" => classes}];
        if Field::Empty == field {
            i.add_style("font-size".into(), "20%".into());
        }
        i
    }

    fn view_cell(&self, col: usize, row: usize) -> El<Message> {
        use seed::*;

        let field = self.model.board.get(col, row);
        let editable = self.model.editable.is_editable(col, row);
        let class = if editable { "guess" } else { "" };
        let id = format!("cell-{}-{}", col, row);
        let size = self.model.get_size();

        let mut td = td![
            // id is required by engine for correct updates,
            // otherwise "board" gets randomized in NewGame (bug in seed?)
            attrs! {"class" => class.to_string(); "id" => id },
            style! {"width" => format!("{}%", 100.0 / (size as f64))},
            self.view_field(field),
        ];
        if editable {
            td.listeners
                .push(simple_ev("click", Message::Toggle(CellPos { col, row })));
        }
        td
    }

    fn view_row(&self, row: usize) -> El<Message> {
        let size = self.model.get_size();
        let cells: Vec<El<Message>> = (0..size).map(|col| self.view_cell(col, row)).collect();
        tr![cells]
    }

    fn view_board(&self) -> El<Message> {
        use seed::*;

        let size = self.model.get_size();
        let rows: Vec<El<Message>> = (0..size).map(|row| self.view_row(row)).collect();
        div![
            attrs! {"id" => "board"},
            if is_board_full(&self.model.board) {
                let valid = is_board_valid(&self.model.board);
                let text = if valid {
                    self.tr("game-won")
                } else {
                    self.tr("game-lost")
                };

                div![
                    attrs! {
                        "class" => if valid { "alert alert-success" } else { "alert alert-danger" };
                        "id" => "end-game-alert"
                    },
                    text
                ]
            } else {
                seed::empty()
            },
            table![rows]
        ]
    }

    fn view_difficulty(&self, difficulty: Difficulty) -> El<Message> {
        use seed::*;

        a![
            attrs! {
                "class" => "dropdown-item";
                "href" => "#";
            },
            self.tr(&format!("difficulty-{}", difficulty)),
            simple_ev("click", Message::NewGame(difficulty))
        ]
    }

    fn view_new_game(&self, difficulty: Difficulty) -> Vec<El<Message>> {
        use seed::*;
        let mut difficulty_arg = HashMap::new();
        difficulty_arg.insert(
            "difficulty",
            FluentValue::String(self.tr(&format!("difficulty-{}", difficulty))),
        );

        seed::log(format!("display difficulty = {}", difficulty));
        seed::log(format!(
            "  has new-game: {}",
            self.bundle.has_message("new-game")
        ));
        seed::log(format!(
            "  has difficulty-display: {}",
            self.bundle.has_message("difficulty-dislay")
        ));
        let text = self
            .bundle
            .format("difficulty-display", Some(&difficulty_arg));
        seed::log(format!("  text = {:#?}", text));
        let diff_header = h4![
            attrs! {"id" => "Difficulty-Display"},
            text.expect(&format!(
                "tr(difficulty-display[difficulty = {}]) failed",
                difficulty
            ))
            .0
        ];
        seed::log("display new-game button");
        let new_game_button = button![
            attrs! {
                "class" => "btn btn-primary dropdown-toggle";
                "type" => "button";
                "id" => "New-Game-Difficulty";
                "data-toggle" => "dropdown";
                "aria-haspopup" => "true";
                "aria-expanded" => "false";
            },
            self.tr("new-game")
        ];
        seed::log("display new-game levels");
        let new_game_levels = div![
            attrs! {
                "class" => "dropdown-menu";
                "aria-labelledby" => "New-Game-Difficulty";
            },
            self.view_difficulty(Difficulty::Easy),
            self.view_difficulty(Difficulty::Medium),
            self.view_difficulty(Difficulty::Hard),
        ];

        seed::log("display new-game combine");
        vec![
            diff_header,
            div![
                attrs! {"class" => "dropdown"},
                new_game_button,
                new_game_levels,
            ],
        ]
    }

    pub fn view(&self) -> El<Message> {
        use seed::*;

        seed::log("before header");
        let header = div![
            attrs! {"class" => "row"},
            div![
                attrs! {"class" => "col"},
                div![
                    attrs! {
                        "class" => "language-switch";
                        "data-toggle" => "tooltip";
                        "data-placement" => "bottom";
                        "title" => self.tr("language-toggle");
                    },
                    i![attrs! {"class" => "fas fa-language"}],
                    simple_ev("click", Message::ToggleLanguage),
                ],
                h1![self.tr("header")],
            ]
        ];
        seed::log("view before board");
        let board = div![
            attrs! {"class" => "cl-xs-8 col-sm-8 col-md-8 col-lg-8"},
            self.view_board()
        ];
        seed::log("view before controls");
        let controls = div![
            attrs! {"class" => "col-xs-4 col-sm-4 col-md-4 col-lg-4"},
            button![
                attrs! {
                    "class" => "btn btn-secondary";
                    "id" => "clear-board"
                },
                self.tr("clear-board"),
                simple_ev("click", Message::Clear)
            ],
            self.view_new_game(self.model.difficulty),
            h4![self.tr("rules-header")],
            ul![
                li![self.tr("rule-1")],
                li![self.tr("rule-2")],
                li![self.tr("rule-3")],
            ]
        ];
        seed::log("view before combine");
        div![
            attrs! {"class" => "container"},
            header,
            div![attrs! {"class" => "row"}, board, controls]
        ]
    }
}

fn build_view<'a>(model: &'a Model) -> ViewBuilder<'a> {
    ViewBuilder {
        bundle: model.res_mgr.get_bundle(&model.language.to_string()),
        model,
    }
}

pub fn view(model: &Model) -> impl ElContainer<Message> {
    let vb = build_view(model);
    vb.view()
}
