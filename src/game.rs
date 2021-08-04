use std::fmt::*;

use crate::{Board, Disc, Position};

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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Game {
    pub turn: Disc,
    pub dark: usize,
    pub light: usize,
    pub empty: usize,
    pub board: Board,
    pub transcript: Vec<Play>,
    pub is_complete: bool,
}

impl Game {
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

    pub fn from_transcript(transcript: &[Play]) -> Option<Self> {
        let mut game = Game::new();
        for p in transcript {
            match p {
                Play::Move(p) => game.play_valid_move(game.validate_move(game.turn, *p)?),
                Play::Pass => game.pass(),
            }
        }

        game.validate_completion();

        Some(game)
    }

    /// Determines if a position is a valid move for the current player.
    pub fn validate_move(&self, player: Disc, position: Position) -> Option<ValidMove> {
        let mut moves_for_empty = self.moves_for_empty(player, position).flatten().collect();
        Self::consolidate_moves(&mut moves_for_empty).pop()
    }

    /// Finds all of the valid moves for the current player.
    #[allow(clippy::manual_flatten)]
    pub fn valid_moves(&self, player: Disc) -> Vec<ValidMove> {
        let mut raw_moves = Vec::with_capacity(64);

        let player_count = match player {
            Disc::Dark => self.dark,
            Disc::Light => self.light,
        };

        // pick our search strategy -- if there are fewer player discs
        // than empty positions, then search for moves starting at those
        // discs, otherwise, search for moves starting in empty positions.
        //
        // Note: these could be implemented with map/flatten/collect,
        // however it's considerably slower.

        if player_count <= self.empty {
            // smaller number of origin discs; search occupied pieces
            for p in self.board.positions_of(Some(player)) {
                for om in self.moves_for_occupied(player, p) {
                    if let Some(m) = om {
                        raw_moves.push(m);
                    }
                }
            }
        } else {
            // smaller number of empty discs; search occupied pieces
            for p in self.board.positions_of(None) {
                for om in self.moves_for_empty(player, p) {
                    if let Some(m) = om {
                        raw_moves.push(m);
                    }
                }
            }
        }

        // merge all of the flips with the same position
        Self::consolidate_moves(&mut raw_moves)
    }

    pub fn play_valid_move(&mut self, valid_move: ValidMove) {
        // do the score accounting before we consume all of the moves
        let flipped = valid_move.flips.len();
        self.empty -= 1; // decrement the empty spaces
        if self.turn == Disc::Dark {
            self.dark += flipped + 1;
            self.light -= flipped;
        } else {
            self.light += flipped + 1;
            self.dark -= flipped;
        }

        // add the disc to the played position
        self.board.set(valid_move.position.into(), self.turn);

        // iterate through the flips and set those discs to the current player
        for p in valid_move.flips {
            self.board.set(p.into(), self.turn);
        }

        // save the played position to the transcript
        self.transcript.push(Play::Move(valid_move.position));

        self.turn = self.turn.opposite();
    }

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

    pub fn validate_completion(&mut self) {
        // if there are no more valid moves for either player, then the game is complete.
        self.is_complete =
            self.valid_moves(Disc::Dark).is_empty() && self.valid_moves(Disc::Light).is_empty()
    }

    /// Returns any valid moves associated with a given occupied position.
    /// They still need to be consolidated; use validate_move()
    fn moves_for_occupied(
        &self,
        player: Disc,
        position: Position,
    ) -> impl Iterator<Item = Option<ValidMove>> + '_ {
        position
            .lines_for()
            .iter()
            .map(move |line| self.move_from_occupied(player, line))
    }

    /// Returns any valid moves associated with a given occupied position.
    /// They still need to be consolidated; use validate_move()
    fn moves_for_empty(
        &self,
        player: Disc,
        position: Position,
    ) -> impl Iterator<Item = Option<ValidMove>> + '_ {
        position
            .lines_for()
            .iter()
            .map(move |line| self.move_from_empty(player, line))
    }

    fn move_from_empty(&self, player: Disc, indexes: &[usize]) -> Option<ValidMove> {
        let mut targets = Vec::with_capacity(6);
        for idx in indexes {
            // ignore the first position; it's not a target, it's where we'll be placing the piece!
            if idx == indexes.first()? {
                continue;
            }

            // no disc? return.
            let disc = self.board.get(*idx)?;

            // possible target disc to flip!
            if disc == player.opposite() {
                targets.push(Position::new(*idx));
                continue;
            }

            // we've found our own color -- if we have targets, it's a valid move!
            if disc == player {
                if targets.is_empty() {
                    return None;
                }
                return Some(ValidMove::new(Position::new(*indexes.first()?), targets));
            }
        }

        None
    }

    fn move_from_occupied(&self, player: Disc, indexes: &[usize]) -> Option<ValidMove> {
        let mut targets = Vec::with_capacity(6);

        for idx in indexes {
            // ignore the first position; it's occupied by the originating position!
            if idx == indexes.first()? {
                continue;
            }

            let disc = self.board.get(*idx);

            // exit if we encounter our own color disc; no playable moves on this line.
            if disc == Some(player) {
                return None;
            }

            // if the position is empty ...
            if disc.is_none() {
                // if we have no targets, there's no line, return none.
                if targets.is_empty() {
                    return None;
                }

                // we have targets! this is a legitimate line; return a valid move!
                return Some(ValidMove::new(Position::new(*idx), targets));
            }

            // if the disc is the opposition color, put the position in targets and move on
            targets.push(Position::new(*idx));
        }

        None
    }

    fn consolidate_moves(valid_moves: &mut Vec<ValidMove>) -> Vec<ValidMove> {
        // sort 'em by position so that the same positions are sequential in the list.
        valid_moves.sort_by(|a, b| a.position.cmp(&b.position));

        // set up our results holding the merged ValidMoves
        let mut output = Vec::with_capacity(valid_moves.len());

        // pop through the initial moves, pushing them to the output, and merging as needed
        while let Some(mut next_vm) = valid_moves.pop() {
            // get the last move in the output, or ensure there's a last!
            let last_vm: &mut ValidMove = match output.last_mut() {
                Some(last_vm) => last_vm,
                None => {
                    output.push(next_vm);
                    continue;
                }
            };

            // if we have a repeat of the same position, take the flips
            // otherwise, push the next_vm
            if last_vm.position == next_vm.position {
                last_vm.take_flips(&mut next_vm);
            } else {
                output.push(next_vm);
            }
        }

        output
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidMove {
    pub position: Position,
    pub flips: Vec<Position>,
}

impl ValidMove {
    pub fn new(position: Position, flips: Vec<Position>) -> Self {
        Self { position, flips }
    }

    pub fn take_flips(&mut self, other: &mut Self) {
        // consume the other flips, and make the set unique
        self.flips.append(&mut other.flips);
        self.flips.sort();
        self.flips.dedup();
    }

    pub fn score(&self) -> usize {
        self.flips.len()
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

        let mut move_positions: Vec<Position> = valid_moves.iter().map(|vm| vm.position).collect();
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

        let bad_position = Position::new(0);
        assert!(game.validate_move(game.turn, bad_position).is_none());

        let good_position = Position::new(19);
        let good_move = game.validate_move(game.turn, good_position);
        let target_move = ValidMove::new(Position::new(19), vec![Position::new(27)]);

        assert_eq!(good_move, Some(target_move));
    }

    #[test]
    fn test_play() {
        // play a few opening moves and check:
        // - turn changes
        // - transcript updates
        // - (println) game state

        let mut game = Game::new();
        println!("{}", game);

        let mut valid_moves = game.valid_moves(game.turn);
        let vm1 = valid_moves.pop().unwrap();
        game.play_valid_move(vm1.clone());

        assert_eq!(game.turn, Disc::Light);
        assert_eq!(game.transcript, vec![Play::Move(vm1.position)]);
        println!("{}", game);

        valid_moves = game.valid_moves(game.turn);
        let vm2 = valid_moves.pop().unwrap();
        game.play_valid_move(vm2.clone());

        assert_eq!(game.turn, Disc::Dark);
        assert_eq!(
            game.transcript,
            vec![Play::Move(vm1.position), Play::Move(vm2.position)]
        );
        println!("{}", game);
    }

    #[test]
    fn test_partial_transcript_round_trip() {
        // test a couple of moves in with an incomplete game
        let mut game = Game::new();

        game.play_valid_move(
            game.validate_move(game.turn, Position::from_xy(3, 2))
                .unwrap(),
        );
        game.play_valid_move(
            game.validate_move(game.turn, Position::from_xy(2, 2))
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
            match game.valid_moves(game.turn).pop() {
                None => game.pass(),
                Some(vm) => game.play_valid_move(vm),
            }
        }

        let game_from_transcript = Game::from_transcript(&game.transcript).unwrap();

        assert_eq!(game_from_transcript, game);
    }

    #[test]
    fn test_scoring() {
        for _ in 0..100 {
            let mut game = Game::new();

            while !game.is_complete {
                match game.valid_moves(game.turn).pop() {
                    None => game.pass(),
                    Some(vm) => game.play_valid_move(vm),
                }
            }

            let dark = game.board.indexes_of(Some(Disc::Dark)).count();
            let light = game.board.indexes_of(Some(Disc::Light)).count();
            let empty = game.board.indexes_of(None).count();

            assert_eq!(game.dark, dark);
            assert_eq!(game.light, light);
            assert_eq!(game.empty, empty);
        }
    }
}
