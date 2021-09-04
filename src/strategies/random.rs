use crate::strategies::{ScoredPlay, Strategies, Strategy};
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

    fn score_plays(&mut self, game: &Game) -> Vec<ScoredPlay> {
        let mut rng = thread_rng();
        game.valid_moves(game.turn)
            .map(|idx| {
                let score = rng.gen_range(0.0..1.0);
                ScoredPlay::new(Strategies::Random, score, idx)
            })
            .collect()
    }
}
