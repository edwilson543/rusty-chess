use super::move_;
use super::translation;

/// Mechanism for defining whether a certain translation is allowed.
pub trait Rule {
    fn allows_move(&self, move_: &move_::Move) -> bool;
}

// Re-usable translation rules.

pub struct SingleSquareMove {
    vector: translation::ChessVector,
}

pub struct MultiSquareMove {
    vector: translation::ChessVector,
}

impl SingleSquareMove {
    pub fn new(vector: translation::ChessVector) -> Self {
        Self { vector }
    }
}

impl Rule for SingleSquareMove {
    fn allows_move(&self, move_: &move_::Move) -> bool {
        let translation = &move_.translation;
        self.vector == translation.vector && translation.scalar == 1 && !translation.is_obstructed()
    }
}

impl MultiSquareMove {
    pub fn new(vector: translation::ChessVector) -> Self {
        Self { vector }
    }
}

impl Rule for MultiSquareMove {
    fn allows_move(&self, move_: &move_::Move) -> bool {
        let translation = &move_.translation;
        self.vector == translation.vector && !translation.is_obstructed()
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod single_square_translation_tests {
        use super::super::*;
        use crate::domain::gameplay::chess_set::{File, Rank, Square};
        use crate::domain::gameplay::rulebook::moves::move_::Move;
        use crate::domain::gameplay::rulebook::moves::translation;
        use crate::testing::factories;

        fn allows_single_square_move() {
            let from_square = Square::new(Rank::ONE, File::A);
            let to_square = Square::new(Rank::ONE, File::B);
            let piece = factories::some_piece();
            let move_ = Move::new(&piece, &from_square, &to_square);

            let vector = translation::ChessVector::new(1, 0);
            let translation_rule = SingleSquareMove::new(vector);

            assert!(translation_rule.allows_move(&move_));
        }

        #[test]
        fn disallows_single_square_move_in_wrong_direction() {
            let from_square = Square::new(Rank::ONE, File::A);
            let to_square = Square::new(Rank::TWO, File::A);
            let piece = factories::some_piece();
            let move_ = Move::new(&piece, &from_square, &to_square);

            let vector = translation::ChessVector::new(1, 0);
            let translation_rule = SingleSquareMove::new(vector);

            assert!(!translation_rule.allows_move(&move_));
        }

        #[test]
        fn disallows_multi_square_translation_matching_vector() {
            let from_square = Square::new(Rank::ONE, File::A);
            let to_square = Square::new(Rank::ONE, File::C);
            let piece = factories::some_piece();
            let move_ = Move::new(&piece, &from_square, &to_square);

            let vector = translation::ChessVector::new(1, 0);
            let translation_rule = SingleSquareMove::new(vector);

            assert!(!translation_rule.allows_move(&move_));
        }
    }

    #[cfg(test)]
    mod multi_square_translation_tests {
        use super::super::*;
        use crate::domain::gameplay::chess_set::{File, Rank, Square};
        use crate::domain::gameplay::rulebook::moves::move_::Move;
        use crate::testing::factories;

        fn allows_multi_square_move() {
            let from_square = Square::new(Rank::ONE, File::A);
            let to_square = Square::new(Rank::ONE, File::C);
            let piece = factories::some_piece();
            let move_ = Move::new(&piece, &from_square, &to_square);

            let vector = translation::ChessVector::new(1, 0);
            let translation_rule = SingleSquareMove::new(vector);

            assert!(translation_rule.allows_move(&move_));
        }

        #[test]
        fn disallows_move_in_wrong_direction() {
            let from_square = Square::new(Rank::ONE, File::A);
            let to_square = Square::new(Rank::THREE, File::C);
            let piece = factories::some_piece();
            let move_ = Move::new(&piece, &from_square, &to_square);

            let vector = translation::ChessVector::new(0, -1);
            let translation_rule = SingleSquareMove::new(vector);

            assert!(!translation_rule.allows_move(&move_));
        }
    }
}
