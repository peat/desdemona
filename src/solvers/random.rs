use crate::solvers::Solver;
use crate::{Game, ValidMove};

use rand::prelude::*;

#[derive(Default)]
pub struct Random {
    rng: rand::rngs::ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Solver for Random {
    fn name(&self) -> &str {
        "Random v1.0"
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        game.valid_moves(game.turn)
            .into_iter()
            .choose(&mut self.rng)
    }
}
