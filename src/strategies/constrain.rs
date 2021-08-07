use crate::strategies::Strategy;
use crate::{Game, ValidMove};

#[derive(Copy, Clone)]
pub struct Constrain {}

impl Strategy for Constrain {
    fn name(&self) -> &str {
        "Constrain"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        // look for the move that blocks the most moves for the opponent
        let mut scored_moves: Vec<(usize, ValidMove)> = game
            .valid_moves(game.turn)
            .into_iter()
            .map(|vm| {
                // clone the game and the move and play it
                let mut possible_game = game.clone();
                possible_game.play_valid_move(vm.clone());

                // count how many moves the opponent has available
                // note: game.turn gets flipped by play()
                let move_count = possible_game.valid_moves(game.turn).len();
                (move_count, vm)
            })
            .collect();

        scored_moves.sort_by(|a, b| a.0.cmp(&b.0));

        // returns None if there isn't one
        let next_move = scored_moves.pop()?.1;

        Some(next_move)
    }
}
