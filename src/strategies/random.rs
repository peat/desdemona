use crate::strategies::Strategy;
use crate::Game;

use rand::prelude::*;

#[derive(Default, Copy, Clone)]
pub struct Random {}

impl Strategy for Random {
    fn name(&self) -> &str {
        "random"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn next_play(&mut self, game: &Game) -> Option<usize> {
        let mut rng = thread_rng();
        game.valid_moves(game.turn).into_iter().choose(&mut rng)
    }
}
