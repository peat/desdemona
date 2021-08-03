mod maximize;
mod minimize;
mod monte;
mod random;
mod simple;

use crate::{Game, ValidMove};
pub use maximize::Maximize;
pub use minimize::Minimize;
pub use monte::Monte;
pub use random::Random;
pub use simple::Simple;

pub trait Solver {
    fn name(&self) -> &str;

    fn solve(&mut self, game: &mut Game) {
        while !game.is_complete {
            match self.next_play(game) {
                Some(valid_move) => game.play_valid_move(valid_move),
                None => game.pass(),
            }
        }
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove>;

    fn bench(&mut self, count: usize) {
        for _ in 0..count {
            let mut game = Game::new();
            self.solve(&mut game);
        }
    }
}
