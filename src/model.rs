use crate::lang::ResourceManager;
use binoxxo::bruteforce::create_puzzle_board;
use binoxxo::field::{Board, Field};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Easy
    }
}

#[derive(Clone, Debug)]
pub struct Editable {
    editable: Vec<bool>,
    size: usize,
}

impl Editable {
    fn new(board: &Board) -> Editable {
        let size = board.get_size();
        let mut editable = vec![false; size * size];

        for x in 0..size {
            for y in 0..size {
                editable[x + size * y] = Field::Empty == board.get(x, y);
            }
        }

        Editable { editable, size }
    }

    pub fn is_editable(&self, x: usize, y: usize) -> bool {
        self.editable[x + self.size * y]
    }
}

#[derive(Debug)]
pub struct Model {
    pub difficulty: Difficulty,
    pub board: Board,
    pub editable: Editable,
    pub res_mgr: ResourceManager,
}

impl Model {
    pub fn new(difficulty: Difficulty) -> Model {
        let (size, guesses) = match difficulty {
            Difficulty::Easy => (6, 5),
            Difficulty::Medium => (8, 10),
            Difficulty::Hard => (10, 15),
        };

        let board = create_puzzle_board(size, guesses);
        let editable = Editable::new(&board);
        let res_mgr = ResourceManager::new();

        Model {
            difficulty,
            board,
            editable,
            res_mgr,
        }
    }

    pub fn get_size(&self) -> usize {
        self.board.get_size()
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::new(Difficulty::default())
    }
}
