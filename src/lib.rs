//! Command line Othello and simulation sandbox!
//!
//! The `desdemona` crate provides a library and binaries for playing, simulating, and
//! developing Othello games and strategies, based on the
//! [official rules](https://www.worldothello.org/about/about-othello/othello-rules/official-rules/english).
//!
//! To play a game on the command line, install the crate and run it:
//!
//! ```bash
//! $ cargo install desdemona
//! ...
//! $ desdemona
//! ```
//!
//! To play and print out a completely random game, you could also do something like this:
//!
//! ```rust
//! use desdemona::strategies::*;
//! use desdemona::Game;
//!
//! // create a new game
//! let mut game = Game::new();
//!
//! // use the Random strategy to solve it
//! Random {}.solve(&mut game);
//!
//! // print the state of the game to the console
//! println!("{}", game);
//!
//! // print out a transcript of all of the moves from the game
//! let plays: Vec<String> = game.transcript.iter().map(|p| format!("{}", p)).collect();
//! println!("Transcript: {}", plays.join(","));
//! ```

/// The 8x8 game board
pub mod board;

/// Internal static data
mod data;

/// Dark or light discs representing played positions
pub mod disc;

/// Game state and rules
pub mod game;

/// Utilities for calculating relative and absolute board positions
pub mod position;

/// Gameplay strategies
pub mod strategies;

pub use board::Board;
pub use disc::Disc;
pub use game::{Game, Play};
pub use position::Position;
