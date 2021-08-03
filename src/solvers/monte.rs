use crate::solvers::{Random, Solver};
use crate::{Disc, Game, ValidMove};

pub struct Monte {
    random: Random,
    name: String,
}

impl Solver for Monte {
    fn name(&self) -> &str {
        &self.name
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        let results: Vec<(usize, ValidMove)> = game
            .valid_moves(game.turn)
            .into_iter()
            .map(|vm| (self.wins_for(&game, &vm), vm))
            .collect();

        let (_, valid_move) = results.into_iter().max_by(|a, b| a.0.cmp(&b.0))?;

        Some(valid_move)
    }
}

impl Monte {
    const ROUNDS: usize = 1_000;

    pub fn new() -> Self {
        Self {
            random: Random::new(),
            name: format!("Monte 1.0 ({} rounds)", Self::ROUNDS),
        }
    }

    fn wins_for(&mut self, game: &Game, valid_move: &ValidMove) -> usize {
        let mut wins = 0;
        for _ in 0..Self::ROUNDS {
            // make a copy of the game
            let mut new_game = game.clone();
            // update it with the given move
            new_game.play_valid_move(valid_move.clone());
            // solve the remainder of plays with the random solver
            self.random.solve(&mut new_game);

            // tally whether this is a win for the current player (the original game.turn)
            let score = new_game.score();
            if ((score.light > score.dark) && game.turn == Disc::Light)
                || ((score.dark > score.light) && game.turn == Disc::Dark)
            {
                wins += 1;
            }
        }
        wins
    }
}
