use crate::solvers::Solver;
use crate::{Game, ValidMove};

pub struct Simple {}

impl Solver for Simple {
    fn name(&self) -> &str {
        "Simple v1.0"
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        game.valid_moves(game.turn).pop()
    }
}
