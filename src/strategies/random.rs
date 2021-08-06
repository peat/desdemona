use crate::strategies::Strategy;
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

impl Strategy for Random {
    fn name(&self) -> &str {
        "Random"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        game.valid_moves(game.turn)
            .into_iter()
            .choose(&mut self.rng)
    }
}
