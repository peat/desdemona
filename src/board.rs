use std::fmt::*;

use crate::{Disc, Position};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Board {
    positions: [Option<Disc>; 64],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            positions: [None; 64],
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board::default();
        board.set(Position::from_xy(3, 3).into(), Disc::Light);
        board.set(Position::from_xy(4, 3).into(), Disc::Dark);
        board.set(Position::from_xy(4, 4).into(), Disc::Light);
        board.set(Position::from_xy(3, 4).into(), Disc::Dark);
        board
    }

    pub fn get(&self, index: usize) -> Option<Disc> {
        self.positions[index]
    }

    pub fn set(&mut self, index: usize, disc: Disc) {
        self.positions[index] = Some(disc)
    }

    pub fn positions_of(&self, disc: Option<Disc>) -> impl Iterator<Item = Position> + '_ {
        self.indexes_of(disc).map(Position::new)
    }

    pub fn indexes_of(&self, disc: Option<Disc>) -> impl Iterator<Item = usize> + '_ {
        self.positions
            .iter()
            .enumerate()
            .filter(move |(_, p)| **p == disc)
            .map(|(idx, _)| idx)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output = String::new();
        output += "  a b c d e f g h\n";
        for row in 0..=7 {
            let index = row * 8;
            let row_characters: Vec<String> = self.positions[index..index + 8]
                .iter()
                .map(|d| match d {
                    None => "·".to_owned(),
                    Some(disc) => format!("{}", disc),
                })
                .collect();

            output += &format!("{} {}\n", row + 1, row_characters.join(" "));
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_board_positions_of() {
        let board = Board::new();
        let empty_indexes = board.indexes_of(None).count();
        let light_indexes = board.indexes_of(Some(Disc::Light)).count();
        let dark_indexes = board.indexes_of(Some(Disc::Dark)).count();

        assert_eq!(empty_indexes, 60);
        assert_eq!(light_indexes, 2);
        assert_eq!(dark_indexes, 2);
    }
}
