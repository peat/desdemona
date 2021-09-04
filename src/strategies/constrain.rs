use crate::strategies::{ScoredPlay, Strategies, Strategy};
use crate::Game;

#[derive(Copy, Clone)]
pub struct Constrain {}

impl Strategy for Constrain {
    fn name(&self) -> &str {
        "constrain"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn score_plays(&mut self, game: &Game) -> Vec<ScoredPlay> {
        let enabled_moves: Vec<(usize, usize)> = game
            .valid_moves(game.turn)
            .map(|index| {
                // clone the game and the move and play it
                let mut possible_game = game.clone();
                possible_game.play(index);

                // count how many moves the opponent has available
                // note: game.turn gets flipped by play()
                let move_count = possible_game.valid_moves(game.turn).count();
                (index, move_count)
            })
            .collect();

        // guard so that we can use unwrap() later
        if enabled_moves.is_empty() {
            return vec![];
        }

        // find the maximum number of enabled moves
        let (_, max_moves) = enabled_moves
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .cloned()
            .unwrap();

        enabled_moves
            .into_iter()
            .map(|(index, moves)| {
                ScoredPlay::new(Strategies::Maximize, moves as f32 / max_moves as f32, index)
            })
            .collect()
    }

    fn next_play(&mut self, game: &Game) -> Option<usize> {
        // look for the move that blocks the most moves for the opponent
        let mut scored_moves: Vec<(usize, usize)> = game
            .valid_moves(game.turn)
            .into_iter()
            .map(|vm| {
                // clone the game and the move and play it
                let mut possible_game = game.clone();
                possible_game.play(vm);

                // count how many moves the opponent has available
                // note: game.turn gets flipped by play()
                let move_count = possible_game.valid_moves(game.turn).count();
                (move_count, vm)
            })
            .collect();

        scored_moves.sort_by(|a, b| a.0.cmp(&b.0));

        // returns None if there isn't one
        let next_move = scored_moves.pop()?.1;

        Some(next_move)
    }
}
