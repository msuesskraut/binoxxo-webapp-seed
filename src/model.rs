use binoxxo::field::{Board, Field};
use binoxxo::bruteforce::create_puzzle_board;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Easy
    }
}

#[derive(Clone, Debug)]
pub struct Editable {
    editable: Vec<bool>,
    size: usize
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

        Editable {
            editable,
            size
        }
    }

    pub fn is_editable(&self, x: usize, y: usize) -> bool {
        self.editable[x + self.size * y]
    }
}
 
#[derive(Clone, Debug)]
pub struct Model {
    pub difficulty: Difficulty,
    pub board: Board,
    pub editable: Editable
}

impl Model {
    fn new(difficulty: Difficulty) -> Model {
        let (size, guesses) = match difficulty {
            Difficulty::Easy => (6, 5),
            Difficulty::Medium => (10, 10),
            Difficulty::Hard => (12, 15),
        };

        let board = create_puzzle_board(size, guesses);
        let editable = Editable::new(&board);

        Model {
            difficulty,
            board,
            editable
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
