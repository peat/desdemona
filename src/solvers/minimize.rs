use crate::solvers::Solver;
use crate::{Game, ValidMove};

pub struct Minimize {}

impl Minimize {
    fn sort(valid_moves: &mut Vec<ValidMove>) -> &mut Vec<ValidMove> {
        valid_moves.sort_by(|a, b| b.score().cmp(&a.score()));
        valid_moves
    }
}

impl Solver for Minimize {
    fn name(&self) -> &str {
        "Minimize v1.0"
    }

    fn next_play(&mut self, game: &Game) -> Option<ValidMove> {
        let mut moves = game.valid_moves(game.turn);
        Self::sort(&mut moves).pop()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Position;

    #[test]
    fn test_moves_sort() {
        let flips1: Vec<usize> = vec![1];
        let flips2: Vec<usize> = vec![1, 2];
        let flips3: Vec<usize> = vec![1, 2, 3];

        let vm1 = ValidMove::new(Position::new(0), vec_usize_to_positions(&flips1));
        let vm2 = ValidMove::new(Position::new(0), vec_usize_to_positions(&flips2));
        let vm3 = ValidMove::new(Position::new(0), vec_usize_to_positions(&flips3));

        let mut sortable = vec![vm2, vm3.clone(), vm1.clone()];
        let sorted = Minimize::sort(&mut sortable);
        assert_eq!(sorted.first(), Some(&vm3));
        assert_eq!(sorted.pop(), Some(vm1));
    }

    fn vec_usize_to_positions(indexes: &[usize]) -> Vec<Position> {
        indexes.iter().map(|i| Position::new(*i)).collect()
    }
}
