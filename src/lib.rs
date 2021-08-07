pub mod board;
mod data;
pub mod disc;
pub mod game;
pub mod position;
pub mod strategies;

pub use board::Board;
pub use disc::Disc;
pub use game::{Game, Play};
pub use position::Position;
