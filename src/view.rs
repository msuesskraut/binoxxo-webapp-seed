use crate::control::{CellPos, Message};
use crate::model::*;
use binoxxo::field::Field;
use binoxxo::rules::{is_board_full, is_board_valid, is_move_valid};
use fluent_bundle::{FluentBundle, FluentValue, FluentResource};
use seed::prelude::*;
use std::collections::HashMap;

struct ViewBuilder<'a> {
    bundle: FluentBundle<&'a FluentResource>,
    model: &'a Model,
}

impl<'a> ViewBuilder<'a> {
    fn tr(&self, id: &str) -> String {
        let mut errors = vec![];
        let msg = self.bundle.get_message(id)
            .expect("Failed to retrieve the message");
        let value = msg.value.expect("Failed to retrieve the value of the message");
        self.bundle
            .format_pattern(value, None, &mut errors).to_string()
    }

    fn view_field(&self, field: Field) -> El<Message> {
        use seed::*;

        let classes = match field {
            Field::Empty => "fas fa-circle",
            Field::X => "fas fa-times",
            Field::O => "far fa-circle",
        };

        let field_view = i![class![classes]];
        if Field::Empty == field {
            field_view.add_style("font-size".into(), "20%".into())
        } else {
            field_view
        }
    }

    fn view_cell(&self, col: usize, row: usize) -> El<Message> {
        use seed::*;

        let field = self.model.board.get(col, row);
        let editable = self.model.editable.is_editable(col, row);
        let is_valid = (Helper::Disabled == self.model.helper)
            || (Field::Empty == field)
            || is_move_valid(&self.model.board, col, row);
        let class_name_guess = if editable { "guess" } else { "" };
        let class_name_valid = if !is_valid { "error" } else { "" };
        let cell_id = format!("cell-{}-{}", col, row);
        let size = self.model.get_size();

        let mut td = td![
            // id is required by engine for correct updates,
            // otherwise "board" gets randomized in NewGame (bug in seed?)
            id!(&cell_id),
            class![class_name_guess, class_name_valid],
            style! {"width" => format!("{}%", 100.0 / (size as f64))},
            self.view_field(field),
        ];
        if editable {
            td.listeners
                .push(simple_ev(Ev::Click, Message::Toggle(CellPos { col, row })));
        }
        td
    }

    fn view_row(&self, row: usize) -> El<Message> {
        use seed::*;

        let size = self.model.get_size();
        let cells: Vec<El<Message>> = (0..size).map(|col| self.view_cell(col, row)).collect();
        tr![cells]
    }

    fn view_difficulty(&self, difficulty: Difficulty) -> El<Message> {
        use seed::*;

        a![
            class!["dropdown-item"],
            attrs! {
                At::Href => "#";
            },
            self.tr(&format!("difficulty-{}", difficulty)),
            simple_ev(Ev::Click, Message::NewGame(difficulty))
        ]
    }

    fn view_new_game_button(&self) -> El<Message> {
        use seed::*;

        let new_game_button = button![
            class!["btn btn-primary dropdown-toggle"],
            id!("New-Game-Difficulty"),
            attrs! {
                At::Type => "button";
                "data-toggle" => "dropdown";
                "aria-haspopup" => "true";
                "aria-expanded" => "false";
            },
            self.tr("new-game")
        ];
        let new_game_levels = div![
            class!["dropdown-menu"],
            attrs! {
                "aria-labelledby" => "New-Game-Difficulty";
            },
            self.view_difficulty(Difficulty::Easy),
            self.view_difficulty(Difficulty::Medium),
            self.view_difficulty(Difficulty::Hard),
        ];
        let enable_helper = button![
            id!("Enable-Disbale-Helper"),
            attrs! {
                "data-toggle" => "tooltip";
                "data-placement" => "right";
                "title" => self.tr("helper-tooltip");
            },
            simple_ev(Ev::Click, Message::ToggleHelper)
        ];
        let enable_helper = enable_helper.add_attr(
            "class".to_string(),
            match self.model.helper {
                Helper::Disabled => "btn btn-outline-secondary",
                Helper::Enabled => "btn btn-secondary",
            }
            .to_string(),
        );
        let enable_helper = enable_helper.add_text(
            &(match self.model.helper {
                Helper::Disabled => self.tr("helper-off"),
                Helper::Enabled => self.tr("helper-on"),
            }),
        );

        div![
            class!["dropdown"],
            new_game_button,
            new_game_levels,
            raw!("&nbsp;"),
            enable_helper
        ]
    }

    fn view_board(&self, board_id: &str, is_error: bool) -> El<Message> {
        use seed::*;

        let size = self.model.get_size();
        let rows: Vec<El<Message>> = (0..size).map(|row| self.view_row(row)).collect();
        div![
            id!(board_id),
            class!["board"],
            table![class![if is_error { "error" } else { "" }], rows,]
        ]
    }

    fn view_new_game_button_success_page(&self, difficulty: Difficulty) -> El<Message> {
        use seed::*;

        button![
            class!["btn btn-primary"],
            self.tr(&format!("difficulty-{}", difficulty)),
            simple_ev(Ev::Click, Message::NewGame(difficulty))
        ]
    }

    fn view_success_alert(&self) -> El<Message> {
        use seed::*;

        div![
            class!["alert alert-success"],
            attrs! {"role" => "alert"},
            h1![class!["alert-heading"], self.tr("game-won")],
            hr![],
            div![
                class!["centered mx-auto"],
                self.view_board("board-success", /*is_error:*/ false)
            ],
            hr![],
            h4![class!["text-center"], self.tr("new-game")],
            div![
                class!["text-center mx-auto"],
                self.view_new_game_button_success_page(Difficulty::Easy),
                raw!["&nbsp;"],
                self.view_new_game_button_success_page(Difficulty::Medium),
                raw!["&nbsp;"],
                self.view_new_game_button_success_page(Difficulty::Hard),
            ]
        ]
    }

    fn view_success_page(&self) -> El<Message> {
        use seed::*;

        div![
            id!("success-page"),
            class!("overlay"),
            div![
                class!("overlay-content"),
                div![
                    class!["container"],
                    div![
                        class!["row justify-content-center"],
                        div![class!["col"], self.view_success_alert()]
                    ]
                ]
            ]
        ]
    }

    fn view_game(&self) -> Vec<El<Message>> {
        let is_full = is_board_full(&self.model.board);
        let is_valid = is_board_valid(&self.model.board);
        let mut game = vec![self.view_board("board", is_full && !is_valid)];
        if is_valid {
            game.push(self.view_success_page());
        }
        game
    }

    fn view_new_game(&self, difficulty: Difficulty) -> Vec<El<Message>> {
        use seed::*;

        let mut difficulty_arg = HashMap::new();
        difficulty_arg.insert(
            "difficulty",
            FluentValue::String(self.tr(&format!("difficulty-{}", difficulty)).into()),
        );

        let mut errors = vec![];
        let msg = self.bundle.get_message("difficulty-display")
            .expect("Failed to retrieve the message");
        let value = msg.value.expect("Failed to retrieve the value of the message");
        let text = self
            .bundle
            .format_pattern(value, Some(&difficulty_arg), &mut errors);
        let diff_header = h4![
            id!("Difficulty-Display"),
            text
        ];

        vec![diff_header, self.view_new_game_button()]
    }

    fn view_footer(&self) -> El<Message> {
        use seed::*;

        const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        const REPO: Option<&'static str> = option_env!("CARGO_PKG_REPOSITORY");

        div![
            class!["row"],
            div![
                class!["col footer"],
                self.tr("app-name"),
                if let Some(url) = REPO {
                    span![
                        " | ",
                        a![
                            attrs! {
                                At::Href => url;
                                At::Rel => "norefferer noopener external";
                                At::Target => "_blank"
                            },
                            "Github"
                        ],
                    ]
                } else {
                    seed::empty()
                },
                format!(
                    " | {}: {}",
                    self.tr("version"),
                    VERSION.unwrap_or(&self.tr("version-unknown"))
                )
            ]
        ]
    }

    pub fn view(&self) -> El<Message> {
        use seed::*;

        let header = div![
            class!["row"],
            div![
                class!["col"],
                div![
                    class!["language-switch"],
                    attrs! {
                        "data-toggle" => "tooltip";
                        "data-placement" => "bottom";
                        At::Title => self.tr("language-toggle");
                    },
                    i![class!["fas fa-language"]],
                    simple_ev(Ev::Click, Message::ToggleLanguage),
                ],
                h1![self.tr("header")],
            ]
        ];
        let board = div![
            class!["cl-xs-8 col-sm-8 col-md-8 col-lg-8"],
            self.view_game()
        ];
        let controls = div![
            class!["col-xs-4 col-sm-4 col-md-4 col-lg-4"],
            button![
                class!["btn btn-secondary"],
                id!("clear-board"),
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
        div![
            class!["container"],
            header,
            div![class!["row"], board, controls],
            self.view_footer()
        ]
    }
}

fn build_view(model: &Model) -> ViewBuilder {
    ViewBuilder {
        bundle: model.res_mgr.get_bundle(&model.language.to_string()),
        model,
    }
}

pub fn view(model: &Model) -> impl ElContainer<Message> {
    let vb = build_view(model);
    vb.view()
}
