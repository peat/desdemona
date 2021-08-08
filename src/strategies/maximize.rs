use crate::strategies::Strategy;
use crate::Game;

#[derive(Clone, Copy)]
pub struct Maximize {}

impl Strategy for Maximize {
    fn name(&self) -> &str {
        "maximize"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn next_play(&mut self, game: &Game) -> Option<usize> {
        let mut moves: Vec<_> = game.valid_moves(game.turn).collect();

        moves.sort_by(|a, b| {
            let a_flips = game.flips_for(*a);
            let b_flips = game.flips_for(*b);
            a_flips.len().cmp(&b_flips.len())
        });

        moves.pop()
    }
}
