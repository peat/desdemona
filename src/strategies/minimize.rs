use crate::strategies::Strategy;
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

    fn next_play(&mut self, game: &Game) -> Option<usize> {
        let mut moves: Vec<_> = game.valid_moves(game.turn).collect();

        moves.sort_by(|a, b| {
            let a_flips = game.flips_for(*a);
            let b_flips = game.flips_for(*b);
            b_flips.len().cmp(&a_flips.len())
        });

        moves.pop()
    }
}
