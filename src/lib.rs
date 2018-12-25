use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {val: model.val + 1}
    }
}


// View

fn view(model: Model) -> El<Msg> {
    use seed::*;

    div![
        button![ 
            attrs!{"class" => "btn btn-light"},
            simple_ev("click", Msg::Increment), 
            format!("Hello, World × {}", model.val) 
        ],
        i![attrs!{"class" => "far fa-square"}],
        i![attrs!{"class" => "fas fa-times"}],
        i![attrs!{"class" => "far fa-circle"}]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);
}