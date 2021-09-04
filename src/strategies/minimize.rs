use crate::strategies::{ScoredPlay, Strategies, Strategy};
use crate::Game;

#[derive(Copy, Clone)]
pub struct Minimize {}

impl Strategy for Minimize {
    fn name(&self) -> &str {
        "minimize"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn score_plays(&mut self, game: &Game) -> Vec<ScoredPlay> {
        let move_flips: Vec<(usize, usize)> = game
            .valid_moves(game.turn)
            .map(|index| (index, game.flips_for(index).len()))
            .collect();

        // guard on empty, so we can do unwraps later on
        if move_flips.is_empty() {
            return vec![];
        }

        let (_, max_flips) = move_flips
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .cloned()
            .unwrap();

        move_flips
            .into_iter()
            .map(|(index, flips)| {
                ScoredPlay::new(
                    Strategies::Minimize,
                    1.0 - (flips as f32 / max_flips as f32),
                    index,
                )
            })
            .collect()
    }
}
