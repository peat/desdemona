use crate::strategies::Strategy;
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

    fn next_play(&mut self, game: &Game) -> Option<usize> {
        game.valid_moves(game.turn).pop()
    }
}
