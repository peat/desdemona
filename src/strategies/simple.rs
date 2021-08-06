use crate::strategies::Strategy;
use crate::{Game, ValidMove};

pub struct Simple {}

impl Strategy for Simple {
    fn name(&self) -> &str {
        "Simple v1.0"
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        game.valid_moves(game.turn).pop()
    }
}
