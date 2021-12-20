use crate::control::{CellPos, Message};
use crate::model::*;
use binoxxo::field::Field;
use binoxxo::rules::{is_board_full, is_board_valid, is_move_valid};
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use seed::{prelude::*, *};
use web_sys::console::log_1;

struct ViewBuilder<'a> {
    bundle: FluentBundle<&'a FluentResource>,
    model: &'a Model,
}

impl<'a> ViewBuilder<'a> {
    fn tr_with_args(&self, id: &str, args: Option<&FluentArgs>) -> String {
        let mut errors = vec![];
        let msg = self
            .bundle
            .get_message(id)
            .expect("Failed to retrieve the message");
        let value = msg
            .value()
            .expect("Failed to retrieve the value of the message");
        let res = self
            .bundle
            .format_pattern(value, args, &mut errors)
            .to_string();
        if !errors.is_empty() {
            log_1(&format!("Failed to translate {}:\n  {:?}\n", id, errors).into());
            panic!("Error in fluent");
        }
        res
    }

    fn tr(&self, id: &str) -> String {
        self.tr_with_args(id, None)
    }

    fn view_field(&self, field: Field) -> Vec<Node<Message>> {
        match field {
            Field::Empty => Node::<Message>::from_html(include_str!("../assets/icons/dot-opt.svg")),
            Field::X => Node::<Message>::from_html(include_str!("../assets/icons/x-opt.svg")),
            Field::O => Node::<Message>::from_html(include_str!("../assets/icons/o-opt.svg")),
        }
    }

    fn view_cell(&self, col: usize, row: usize) -> Node<Message> {
        let field = self.model.board.get(col, row);
        let editable = self.model.editable.is_editable(col, row);
        let is_valid = (Helper::Disabled == self.model.helper)
            || (Field::Empty == field)
            || is_move_valid(&self.model.board, col, row);
        let cell_id = format!("cell-{}-{}", col, row);
        let size = self.model.get_size();

        td![
            // id is required by engine for correct updates,
            // otherwise "board" gets randomized in NewGame (bug in seed?)
            id!(&cell_id),
            C![IF!(editable => "guess"), IF!(not(is_valid) => "error")],
            style! {St::Width => format!("{}%", 100.0 / (size as f64))},
            self.view_field(field),
            IF!(editable => ev(Ev::Click, move |_| { Message::Toggle(CellPos { col, row }) }))
        ]
    }

    fn view_row(&self, row: usize) -> Node<Message> {
        let size = self.model.get_size();
        let cells: Vec<Node<Message>> = (0..size).map(|col| self.view_cell(col, row)).collect();
        tr![cells]
    }

    fn view_difficulty(&self, difficulty: Difficulty) -> Node<Message> {
        a![
            C!["dropdown-item"],
            attrs! {
                At::Href => "#";
            },
            self.tr(&format!("difficulty-{}", difficulty)),
            ev(Ev::Click, move |_| { Message::NewGame(difficulty) })
        ]
    }

    fn view_new_game_button(&self) -> Node<Message> {
        let new_game_button = button![
            C!["btn btn-primary dropdown-toggle"],
            id!("New-Game-Difficulty"),
            attrs! {
                At::Type => "button";
                "data-toggle" => "dropdown";
                At::AriaHasPopup => "true";
                At::AriaExpanded => "false";
            },
            self.tr("new-game")
        ];
        let new_game_levels = div![
            C!["dropdown-menu"],
            attrs! {
                At::AriaLabelledBy => "New-Game-Difficulty";
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
                At::Title => self.tr("helper-tooltip");
            },
            C! {
                match self.model.helper {
                    Helper::Disabled => "btn btn-outline-secondary",
                    Helper::Enabled => "btn btn-secondary",
                }
            },
            match self.model.helper {
                Helper::Disabled => self.tr("helper-off"),
                Helper::Enabled => self.tr("helper-on"),
            },
            ev(Ev::Click, |_| { Message::ToggleHelper })
        ];

        div![
            C!["dropdown"],
            new_game_button,
            new_game_levels,
            raw!("&nbsp;"),
            enable_helper
        ]
    }

    fn view_board(&self, board_id: &str, is_error: bool) -> Node<Message> {
        let size = self.model.get_size();
        let rows: Vec<Node<Message>> = (0..size).map(|row| self.view_row(row)).collect();
        div![
            id!(board_id),
            C!["board"],
            table![C![if is_error { "error" } else { "" }], rows,]
        ]
    }

    fn view_new_game_button_success_page(&self, difficulty: Difficulty) -> Node<Message> {
        button![
            C!["btn btn-primary"],
            self.tr(&format!("difficulty-{}", difficulty)),
            ev(Ev::Click, move |_| { Message::NewGame(difficulty) })
        ]
    }

    fn view_success_alert(&self) -> Node<Message> {
        div![
            C!["alert alert-success"],
            attrs! {"role" => "alert"},
            h1![C!["alert-heading"], self.tr("game-won")],
            hr![],
            div![
                C!["centered mx-auto"],
                self.view_board("board-success", /*is_error:*/ false)
            ],
            hr![],
            h4![C!["text-center"], self.tr("new-game")],
            div![
                C!["text-center mx-auto"],
                self.view_new_game_button_success_page(Difficulty::Easy),
                raw!["&nbsp;"],
                self.view_new_game_button_success_page(Difficulty::Medium),
                raw!["&nbsp;"],
                self.view_new_game_button_success_page(Difficulty::Hard),
            ]
        ]
    }

    fn view_success_page(&self) -> Node<Message> {
        div![
            id!("success-page"),
            C!("overlay"),
            div![
                C!("overlay-content"),
                div![
                    C!["container"],
                    div![
                        C!["row justify-content-center"],
                        div![C!["col"], self.view_success_alert()]
                    ]
                ]
            ]
        ]
    }

    fn view_game(&self) -> Vec<Node<Message>> {
        let is_full = is_board_full(&self.model.board);
        let is_valid = is_board_valid(&self.model.board);
        nodes![
            self.view_board("board", is_full && !is_valid),
            IF!(is_valid => self.view_success_page())
        ]
    }

    fn view_new_game(&self, difficulty: Difficulty) -> Vec<Node<Message>> {
        // build arguments for translation difficulty-display
        let mut difficulty_arg = FluentArgs::new();
        difficulty_arg.set(
            "difficulty",
            FluentValue::from(self.tr(&format!("difficulty-{}", difficulty))),
        );

        let text = self.tr_with_args("difficulty-display", Some(&difficulty_arg));

        let diff_header = h4![id!("Difficulty-Display"), text];

        nodes![diff_header, self.view_new_game_button()]
    }

    fn view_footer(&self) -> Node<Message> {
        const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        const REPO: Option<&'static str> = option_env!("CARGO_PKG_REPOSITORY");

        div![
            C!["row"],
            div![
                C!["col footer"],
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

    pub fn view(&self) -> Node<Message> {
        let header = div![
            C!["row"],
            div![
                C!["col"],
                div![
                    C!["language-switch"],
                    attrs! {
                        "data-toggle" => "tooltip";
                        "data-placement" => "bottom";
                        At::Title => self.tr("language-toggle");
                    },
                    img![attrs! {
                        At::Src => "translate.svg",
                        At::Height => "300%",
                        At::Width => "300%",
                    }],
                    ev(Ev::Click, |_| { Message::ToggleLanguage }),
                ],
                h1![self.tr("header")],
            ]
        ];
        let board = div![C!["cl-xs-8 col-sm-8 col-md-8 col-lg-8"], self.view_game()];
        let controls = div![
            C!["col-xs-4 col-sm-4 col-md-4 col-lg-4"],
            button![
                C!["btn btn-secondary"],
                id!("clear-board"),
                self.tr("clear-board"),
                ev("click", |_| { Message::Clear })
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
            C!["container"],
            header,
            div![C!["row"], board, controls],
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

pub fn view(model: &Model) -> Node<Message> {
    let vb = build_view(model);
    vb.view()
}
