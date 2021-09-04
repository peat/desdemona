use crate::strategies::Strategy;
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

    fn next_play(&mut self, game: &Game) -> Option<usize> {
        let mut all_moves: Vec<_> = game.valid_moves(game.turn).collect();

        // check if any of the available moves are in the true corners
        let mut best_moves: Vec<usize> = all_moves
            .iter()
            .filter(|idx| GOOD_CORNER_INDEXES.contains(idx))
            .cloned()
            .collect();

        // if so, play the first one!
        if !best_moves.is_empty() {
            return best_moves.pop();
        }

        // if not, strip the bad corner positions from what's left
        let mut bad_removed: Vec<usize> = all_moves
            .iter()
            .filter(|idx| !BAD_CORNER_INDEXES.contains(idx))
            .cloned()
            .collect();

        // if we have moves that aren't bad corners, play the first one
        if !bad_removed.is_empty() {
            return bad_removed.pop();
        }

        // no other option!
        all_moves.pop()
    }
}
