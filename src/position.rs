use std::fmt::*;

const MAX_XY: usize = 7;
const MAX_INDEX: usize = 63;

const X_POSITIONS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const Y_POSITIONS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

/// Provides convenience methods for translating from coordinates to indexes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Position(usize);

impl Position {
    pub fn new(index: usize) -> Self {
        if index > MAX_INDEX {
            panic!("Position out of bounds - index: {}", index);
        }

        Self(index)
    }

    pub fn from_xy(x: usize, y: usize) -> Self {
        if x > MAX_XY || y > MAX_XY {
            panic!("Position out of bounds - x: {}, y: {}", x, y);
        }

        Self::new((y * 8) + x)
    }

    pub fn to_xy(self) -> (usize, usize) {
        let x = self.0 % 8;
        let y = self.0 / 8;

        (x, y)
    }

    pub fn generate_north(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index >= 8 {
            index -= 8;
            output.push(Position::new(index));
        }
        output
    }

    pub fn generate_north_east(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index >= 8 && index % 8 != 7 {
            index -= 7;
            output.push(Position::new(index));
        }
        output
    }

    pub fn generate_east(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index % 8 != 7 {
            index += 1;
            output.push(Position::new(index));
        }
        output
    }

    pub fn generate_south_east(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index % 8 != 7 && index <= 54 {
            index += 9;
            output.push(Position::new(index));
        }
        output
    }

    pub fn generate_south(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index <= 55 {
            index += 8;
            output.push(Position::new(index));
        }
        output
    }

    pub fn generate_south_west(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index <= 56 && index % 8 != 0 {
            index += 7;
            output.push(Position::new(index));
        }
        output
    }

    pub fn generate_west(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index % 8 != 0 && index >= 1 {
            index -= 1;
            output.push(Position::new(index));
        }
        output
    }

    pub fn generate_north_west(&self) -> Vec<Position> {
        let mut output = Vec::with_capacity(8);
        output.push(*self);
        let mut index = self.0;
        while index % 8 != 0 && index >= 9 {
            index -= 9;
            output.push(Position::new(index));
        }
        output
    }
}

impl From<Position> for usize {
    fn from(position: Position) -> Self {
        position.0
    }
}

impl From<&Position> for usize {
    fn from(position: &Position) -> Self {
        position.0
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (x, y) = self.to_xy();
        write!(f, "{}{}", X_POSITIONS[x], Y_POSITIONS[y])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_xy_index() {
        let equivalent_xy_indexes = [((0, 0), 0), ((1, 0), 1), ((1, 1), 9), ((7, 7), 63)];

        for ((x, y), target_index) in equivalent_xy_indexes {
            let position_from_xy = Position::from_xy(x, y);
            let position_from_index = Position::new(target_index);

            // these positions should be the same
            assert_eq!(position_from_xy, position_from_index);
        }
    }

    #[test]
    fn test_lines() {
        // test the results of every corner in every direction. Sheesh!
        let position = Position::new(0);
        test_list(&position.generate_north(), vec![0]);
        test_list(&position.generate_north_east(), vec![0]);
        test_list(&position.generate_east(), vec![0, 1, 2, 3, 4, 5, 6, 7]);
        test_list(
            &position.generate_south_east(),
            vec![0, 9, 18, 27, 36, 45, 54, 63],
        );
        test_list(
            &position.generate_south(),
            vec![0, 8, 16, 24, 32, 40, 48, 56],
        );
        test_list(&position.generate_south_west(), vec![0]);
        test_list(&position.generate_west(), vec![0]);
        test_list(&position.generate_north_west(), vec![0]);

        let position = Position::new(7);
        test_list(&position.generate_north(), vec![7]);
        test_list(&position.generate_north_east(), vec![7]);
        test_list(&position.generate_east(), vec![7]);
        test_list(&position.generate_south_east(), vec![7]);
        test_list(
            &position.generate_south(),
            vec![7, 15, 23, 31, 39, 47, 55, 63],
        );
        test_list(
            &position.generate_south_west(),
            vec![7, 14, 21, 28, 35, 42, 49, 56],
        );
        test_list(&position.generate_west(), vec![7, 6, 5, 4, 3, 2, 1, 0]);
        test_list(&position.generate_north_west(), vec![7]);

        let position = Position::new(56);
        test_list(
            &position.generate_north(),
            vec![56, 48, 40, 32, 24, 16, 8, 0],
        );
        test_list(
            &position.generate_north_east(),
            vec![56, 49, 42, 35, 28, 21, 14, 7],
        );
        test_list(
            &position.generate_east(),
            vec![56, 57, 58, 59, 60, 61, 62, 63],
        );
        test_list(&position.generate_south_east(), vec![56]);
        test_list(&position.generate_south(), vec![56]);
        test_list(&position.generate_south_west(), vec![56]);
        test_list(&position.generate_west(), vec![56]);
        test_list(&position.generate_north_west(), vec![56]);

        let position = Position::new(63);
        test_list(
            &position.generate_north(),
            vec![63, 55, 47, 39, 31, 23, 15, 7],
        );
        test_list(&position.generate_north_east(), vec![63]);
        test_list(&position.generate_east(), vec![63]);
        test_list(&position.generate_south_east(), vec![63]);
        test_list(&position.generate_south(), vec![63]);
        test_list(&position.generate_south_west(), vec![63]);
        test_list(
            &position.generate_west(),
            vec![63, 62, 61, 60, 59, 58, 57, 56],
        );
        test_list(
            &position.generate_north_west(),
            vec![63, 54, 45, 36, 27, 18, 9, 0],
        );

        // throw a center position in there for good measure
        let position = Position::new(27);
        test_list(&position.generate_north(), vec![27, 19, 11, 3]);
        test_list(&position.generate_north_east(), vec![27, 20, 13, 6]);
        test_list(&position.generate_east(), vec![27, 28, 29, 30, 31]);
        test_list(&position.generate_south_east(), vec![27, 36, 45, 54, 63]);
        test_list(&position.generate_south(), vec![27, 35, 43, 51, 59]);
        test_list(&position.generate_south_west(), vec![27, 34, 41, 48]);
        test_list(&position.generate_west(), vec![27, 26, 25, 24]);
        test_list(&position.generate_north_west(), vec![27, 18, 9, 0]);
    }

    fn get_indices(list: &[Position]) -> Vec<usize> {
        list.iter().map(|p| p.0).collect()
    }

    fn test_list(a: &[Position], b: Vec<usize>) {
        assert_eq!(get_indices(a), b);
    }
}
