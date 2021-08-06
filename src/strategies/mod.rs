mod constrain;
mod maximize;
mod minimize;
mod monte;
mod random;
mod simple;

use crate::{Game, ValidMove};
pub use constrain::Constrain;
pub use maximize::Maximize;
pub use minimize::Minimize;
pub use monte::Monte;
pub use random::Random;
pub use simple::Simple;

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub enum Strategies {
    Constrain,
    Maximize,
    Minimize,
    Monte,
    Random,
    Simple,
}

impl Strategies {
    pub fn from_str(name: &str) -> Option<Box<dyn Strategy>> {
        match name {
            "constrain" => Some(Box::new(Constrain {})),
            "maximize" => Some(Box::new(Maximize {})),
            "minimize" => Some(Box::new(Minimize {})),
            "monte" => Some(Box::new(Monte {})),
            "random" => Some(Box::new(Random::new())),
            "simple" => Some(Box::new(Simple {})),
            _ => None,
        }
    }

    pub fn all() -> HashMap<Strategies, Box<dyn Strategy>> {
        let mut output: HashMap<Strategies, Box<dyn Strategy>> = HashMap::new();

        output.insert(Strategies::Constrain, Box::new(Constrain {}));
        output.insert(Strategies::Maximize, Box::new(Maximize {}));
        output.insert(Strategies::Minimize, Box::new(Minimize {}));
        output.insert(Strategies::Monte, Box::new(Monte {}));
        output.insert(Strategies::Random, Box::new(Random::new()));
        output.insert(Strategies::Simple, Box::new(Simple {}));

        output
    }
}

pub trait Strategy {
    fn name(&self) -> &str;
    fn version(&self) -> &str;

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
