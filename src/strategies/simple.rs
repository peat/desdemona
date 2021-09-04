use crate::strategies::{ScoredPlay, Strategies, Strategy};
use crate::Game;

#[derive(Copy, Clone)]
pub struct Simple {}

impl Strategy for Simple {
    fn name(&self) -> &str {
        "simple"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn score_plays(&mut self, game: &Game) -> Vec<ScoredPlay> {
        let valid_moves: Vec<usize> = game.valid_moves(game.turn).collect();
        // first move is scored 1.0, the rest are 0.5
        valid_moves
            .iter()
            .map(|idx| {
                // won't reach this unless theres a first(), so ...
                if idx == valid_moves.first().unwrap() {
                    ScoredPlay::new(Strategies::Simple, 1.0, *idx)
                } else {
                    ScoredPlay::new(Strategies::Simple, 0.5, *idx)
                }
            })
            .collect()
    }
}
