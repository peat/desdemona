use crate::{Board, Disc, Position};
use std::fmt::*;

/// Represents individual plays within the transcript of a game
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Play {
    Move(Position),
    Pass,
}

impl Display for Play {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Play::Move(p) => write!(f, "{}", p),
            Play::Pass => write!(f, "p"),
        }
    }
}

/// Represents the current state of a game in progress. The board positions are indexed from 0 to 63.
/// See [Board] for more details on how this works.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Game {
    /// Which player is currently playing
    pub turn: Disc,
    /// The score of the dark player
    pub dark: usize,
    /// The score of the light player
    pub light: usize,
    /// The number of empty positions on the board
    pub empty: usize,
    /// The game board
    pub board: Board,
    /// The transcript of the game played so far
    pub transcript: Vec<Play>,
    /// Whether or not the game is complete
    pub is_complete: bool,
}

impl Game {
    /// Returns a new game, with the four initial discs placed, and turn set to dark
    pub fn new() -> Self {
        let turn = Disc::Dark;
        let board = Board::new();
        let transcript = Vec::with_capacity(64);
        let is_complete = false;

        Self {
            turn,
            dark: 2,
            light: 2,
            empty: 60,
            board,
            transcript,
            is_complete,
        }
    }

    /// Replays a game from a transcript, returning `None` if there's an error.
    pub fn from_transcript(transcript: &[Play]) -> Option<Self> {
        let mut game = Game::new();
        for p in transcript {
            match p {
                Play::Move(p) => game.play(game.validate_move(game.turn, p.into())?),
                Play::Pass => game.pass(),
            };
        }

        game.validate_completion();

        Some(game)
    }

    /// Determines if a position is a valid move for the current player. Returns the index if so.
    pub fn validate_move(&self, player: Disc, index: usize) -> Option<usize> {
        if self.can_move(player, index) {
            return Some(index);
        }

        None
    }

    /// Returns an `Iterator` with all of the valid moves for the current player.
    pub fn valid_moves(&self, player: Disc) -> impl Iterator<Item = usize> + '_ {
        self.board
            .indexes_of(None)
            .filter(move |index| self.can_move(player, *index))
    }

    /// Returns the position of the discs that _would_ be flipped by playing at a given position.
    pub fn flips_for(&self, index: usize) -> Vec<usize> {
        let player = self.turn;
        let opposition = self.turn.opposite();

        // reusable buffer for collecting lines
        let mut line_flips = Vec::with_capacity(8);

        // all flips for this particular move
        let mut flips = Vec::with_capacity(32);

        for line in self.board.lines_for(index) {
            line_flips.clear();
            for line_index in line.iter() {
                // skip the position being analyzed
                if line_index == &index {
                    continue;
                }

                let disc = self.board.get(*line_index);

                // dead end!
                if disc == None {
                    break;
                }

                // possible target disc to flip!
                if disc == Some(opposition) {
                    line_flips.push(*line_index);
                    continue;
                }

                // we've found our own color -- if we have targets, it's a valid move!
                if disc == Some(player) {
                    // nothing buffered? nothing to flip. next line!
                    if line_flips.is_empty() {
                        break;
                    }

                    // we've buffered some line flips; add them to the over all list of flips
                    flips.append(&mut line_flips);
                    break;
                }
            }
        }

        flips
    }

    /// Plays a disc at the given index for the current player, flipping opposition discs
    /// and updating the score.
    pub fn play(&mut self, index: usize) {
        // find the flips for playing that position
        let mut flips = self.flips_for(index);

        // save this for scoring updates
        let changed = flips.len();

        // add the position to discs to set
        flips.push(index);

        // iterate through the flips and set those discs to the current player
        for p in flips.into_iter() {
            self.board.set(p, self.turn);
        }

        // save the played position to the transcript
        self.transcript.push(Play::Move(Position::new(index)));

        // update the score and completeness
        if self.turn == Disc::Dark {
            self.dark += changed + 1; // include the newly placed piece
            self.light -= changed; // all of the flipped discs
        } else {
            self.light += changed + 1; // include the newly placed piece
            self.dark -= changed; // all of the flipped discs
        }

        self.empty -= 1;

        if self.empty == 0 {
            self.is_complete = true;
        }

        // update the turn
        self.turn = self.turn.opposite();
    }

    /// Forfeits a turn for the current player
    pub fn pass(&mut self) {
        // if the last (opponent) play was also a pass, then the game is over.
        if self.transcript.last() == Some(&Play::Pass) {
            self.is_complete = true;
        } else {
            // otherwise, mark the pass and move on to the other player!
            self.transcript.push(Play::Pass);
            self.turn = self.turn.opposite();
        }
    }

    fn validate_completion(&mut self) {
        // if there are no more valid moves for either player, then the game is complete.
        self.is_complete =
            self.valid_moves(Disc::Dark).count() == 0 && self.valid_moves(Disc::Light).count() == 0
    }

    fn can_move(&self, player: Disc, index: usize) -> bool {
        // basic check ... occupied?
        if self.board.get(index).is_some() {
            return false;
        }

        for line in self.board.lines_for(index) {
            if self.move_from_empty(player, line).is_some() {
                return true;
            }
        }

        false
    }

    fn move_from_empty(&self, player: Disc, indexes: &[usize]) -> Option<usize> {
        let mut line_started = false;

        for idx in indexes {
            // ignore the first position; it's not a target, it's where we'll be placing the piece!
            if idx == indexes.first()? {
                continue;
            }

            // no disc? return.
            let disc = self.board.get(*idx)?;

            // possible target disc to flip!
            if disc == player.opposite() {
                line_started = true;
                continue;
            }

            // we've found our own color -- if we have targets, it's a valid move!
            if disc == player {
                if !line_started {
                    return None;
                }

                return indexes.first().cloned();
            }
        }

        None
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "{}\nTurn: {} Dark: {} Light: {} Empty: {}",
            self.board, self.turn, self.dark, self.light, self.empty,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_moves() {
        // test valid_moves for the starting board.
        let game = Game::new();
        let valid_moves = game.valid_moves(game.turn);

        let mut move_positions: Vec<Position> =
            valid_moves.into_iter().map(Position::new).collect();
        move_positions.sort(); // ensure they're in a predictable order

        let mut target_move_positions = vec![
            Position::from_xy(2, 3),
            Position::from_xy(3, 2),
            Position::from_xy(5, 4),
            Position::from_xy(4, 5),
        ];
        target_move_positions.sort();

        assert_eq!(move_positions, target_move_positions);
    }

    #[test]
    fn test_validate_move() {
        let game = Game::new();

        assert!(game.validate_move(game.turn, 0).is_none());

        let good_index = game.validate_move(game.turn, 19);

        assert_eq!(good_index.unwrap(), 19);
    }

    #[test]
    fn test_play() {
        // play a few opening moves and check:
        // - turn changes
        // - transcript updates
        // - (println) game state

        let mut game = Game::new();

        let valid_move1 = game.valid_moves(game.turn).next().unwrap();
        game.play(valid_move1);

        assert_eq!(game.turn, Disc::Light);
        assert_eq!(
            game.transcript,
            vec![Play::Move(Position::new(valid_move1))]
        );

        let valid_move2 = game.valid_moves(game.turn).next().unwrap();
        game.play(valid_move2);

        assert_eq!(game.turn, Disc::Dark);
        assert_eq!(
            game.transcript,
            vec![
                Play::Move(Position::new(valid_move1)),
                Play::Move(Position::new(valid_move2))
            ]
        );
    }

    #[test]
    fn test_partial_transcript_round_trip() {
        // test a couple of moves in with an incomplete game
        let mut game = Game::new();

        game.play(
            game.validate_move(game.turn, Position::from_xy(3, 2).into())
                .unwrap(),
        );
        game.play(
            game.validate_move(game.turn, Position::from_xy(2, 2).into())
                .unwrap(),
        );

        let game_from_transcript = Game::from_transcript(&game.transcript).unwrap();

        assert_eq!(game_from_transcript, game);
    }

    #[test]
    fn test_complete_transcript_round_trip() {
        // test a complete game
        let mut game = Game::new();

        while !game.is_complete {
            let vm = game.valid_moves(game.turn).next();
            match vm {
                None => game.pass(),
                Some(vm) => game.play(vm),
            };
        }

        let game_from_transcript = Game::from_transcript(&game.transcript).unwrap();

        assert_eq!(game_from_transcript, game);
    }

    #[test]
    fn test_scoring() {
        let mut game = Game::new();

        while !game.is_complete {
            let vm = game.valid_moves(game.turn).next();
            match vm {
                None => game.pass(),
                Some(vm) => game.play(vm),
            };
        }

        let dark = game.board.indexes_of(Some(Disc::Dark)).count();
        let light = game.board.indexes_of(Some(Disc::Light)).count();
        let empty = game.board.indexes_of(None).count();

        assert_eq!(game.dark, dark);
        assert_eq!(game.light, light);
        assert_eq!(game.empty, empty);
    }
}
