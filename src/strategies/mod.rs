mod constrain;
mod corners;
mod maximize;
mod minimize;
mod monte;
mod random;
mod simple;

use crate::Game;
pub use constrain::Constrain;
pub use corners::Corners;
pub use maximize::Maximize;
pub use minimize::Minimize;
pub use monte::Monte;
pub use random::Random;
pub use simple::Simple;

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub enum Strategies {
    Constrain,
    Corners,
    Maximize,
    Minimize,
    Monte,
    Random,
    Simple,
}

impl Strategies {
    pub fn from_name(name: &str) -> Option<Box<dyn Strategy>> {
        match name {
            "constrain" => Some(Box::new(Constrain {})),
            "corners" => Some(Box::new(Corners {})),
            "maximize" => Some(Box::new(Maximize {})),
            "minimize" => Some(Box::new(Minimize {})),
            "monte" => Some(Box::new(Monte {})),
            "random" => Some(Box::new(Random {})),
            "simple" => Some(Box::new(Simple {})),
            _ => None,
        }
    }

    pub fn all() -> HashMap<Strategies, Box<dyn Strategy>> {
        let mut output: HashMap<Strategies, Box<dyn Strategy>> = HashMap::new();

        output.insert(Strategies::Constrain, Box::new(Constrain {}));
        output.insert(Strategies::Corners, Box::new(Corners {}));
        output.insert(Strategies::Maximize, Box::new(Maximize {}));
        output.insert(Strategies::Minimize, Box::new(Minimize {}));
        output.insert(Strategies::Monte, Box::new(Monte {}));
        output.insert(Strategies::Random, Box::new(Random {}));
        output.insert(Strategies::Simple, Box::new(Simple {}));

        output
    }
}

pub trait Strategy: Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;

    fn solve(&mut self, game: &mut Game) {
        while !game.is_complete {
            match self.next_play(game) {
                Some(valid_move) => game.play(valid_move),
                None => game.pass(),
            };
        }
    }

    fn next_play(&mut self, game: &Game) -> Option<usize> {
        let max_play = self.score_plays(game).iter().max()?.index;
        Some(max_play)
    }

    fn score_plays(&mut self, game: &Game) -> Vec<ScoredPlay>;

    fn bench(&mut self, count: usize) {
        for _ in 0..count {
            let mut game = Game::new();
            self.solve(&mut game);
        }
    }
}

pub struct ScoredPlay {
    pub strategy: Strategies,
    pub score: f32,
    pub index: usize,
}

impl ScoredPlay {
    pub fn new(strategy: Strategies, score: f32, index: usize) -> Self {
        Self {
            strategy,
            score,
            index,
        }
    }
}

impl Eq for ScoredPlay {}

impl Ord for ScoredPlay {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // yolo comparing floats
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl PartialOrd for ScoredPlay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScoredPlay {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
