use crate::strategies::{ScoredPlay, Strategies, Strategy};
use crate::Game;

#[derive(Copy, Clone)]
pub struct Corners {}

const GOOD_CORNER_INDEXES: [usize; 4] = [0, 7, 56, 63];
const BAD_CORNER_INDEXES: [usize; 12] = [1, 8, 6, 9, 14, 15, 48, 49, 54, 55, 57, 62];

impl Strategy for Corners {
    fn name(&self) -> &str {
        "corners"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn score_plays(&mut self, game: &Game) -> Vec<ScoredPlay> {
        // Good corners = 1.0
        // Bad corners = 0.0
        // Everything else = 0.5
        game.valid_moves(game.turn)
            .map(|index| {
                if GOOD_CORNER_INDEXES.contains(&index) {
                    ScoredPlay::new(Strategies::Corners, 1.0, index)
                } else if BAD_CORNER_INDEXES.contains(&index) {
                    ScoredPlay::new(Strategies::Corners, 0.0, index)
                } else {
                    ScoredPlay::new(Strategies::Corners, 0.5, index)
                }
            })
            .collect()
    }
}
