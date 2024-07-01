use super::super::move_rule;
use super::super::translation;

pub struct SingleSquareMove {
    vector: translation::ChessVector,
}

impl SingleSquareMove {
    pub fn new(vector: translation::ChessVector) -> Self {
        Self { vector }
    }
}

impl move_rule::MoveRule for SingleSquareMove {
    fn allows_move(&self, move_: &move_rule::Move) -> bool {
        let translation = &move_.translation;
        self.vector == translation.vector && translation.scalar == 1 && !translation.is_obstructed()
    }
}

#[cfg(test)]
mod single_square_translation_tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::move_rule::{Move, MoveRule};
    use crate::domain::gameplay::rulebook::moves::translation;
    use crate::testing::factories;

    #[test]
    fn allows_single_square_move_white() {
        let from_square = Square::new(Rank::One, File::A);
        let to_square = Square::new(Rank::Two, File::A);
        let piece = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &piece, &from_square, &to_square);

        let vector = translation::ChessVector::new(0, 1);
        let rule = SingleSquareMove::new(vector);

        assert!(rule.allows_move(&move_));
    }

    #[test]
    fn allows_single_square_move_black() {
        let from_square = Square::new(Rank::Two, File::A);
        let to_square = Square::new(Rank::One, File::A);
        let piece = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &piece, &from_square, &to_square);

        let vector = translation::ChessVector::new(0, 1);
        let rule = SingleSquareMove::new(vector);

        assert!(rule.allows_move(&move_));
    }

    #[test]
    fn disallows_single_square_move_in_wrong_direction() {
        let from_square = Square::new(Rank::One, File::A);
        let to_square = Square::new(Rank::Two, File::A);
        let piece = factories::some_piece();

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &piece, &from_square, &to_square);

        let vector = translation::ChessVector::new(1, 0);
        let rule = SingleSquareMove::new(vector);

        assert!(!rule.allows_move(&move_));
    }

    #[test]
    fn disallows_multi_square_translation_matching_vector() {
        let from_square = Square::new(Rank::One, File::A);
        let to_square = Square::new(Rank::One, File::C);
        let piece = factories::some_piece();

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &piece, &from_square, &to_square);

        let vector = translation::ChessVector::new(1, 0);
        let rule = SingleSquareMove::new(vector);

        assert!(!rule.allows_move(&move_));
    }
}
