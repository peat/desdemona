use crate::strategies::Strategy;
use crate::{Game, ValidMove};

#[derive(Copy, Clone)]
pub struct Simple {}

impl Strategy for Simple {
    fn name(&self) -> &str {
        "Simple"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        game.valid_moves(game.turn).pop()
    }
}
