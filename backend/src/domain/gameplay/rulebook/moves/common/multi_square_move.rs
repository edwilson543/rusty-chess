use super::super::move_rule;
use super::super::translation;

pub struct MultiSquareMove {
    vector: translation::ChessVector,
}

impl MultiSquareMove {
    pub fn new(vector: translation::ChessVector) -> Self {
        Self { vector }
    }
}

impl move_rule::MoveRule for MultiSquareMove {
    fn allows_move(&self, move_: &move_rule::Move) -> bool {
        let translation = &move_.translation;
        self.vector == translation.vector && !translation.is_obstructed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::move_rule::{Move, MoveRule};
    use crate::domain::gameplay::rulebook::moves::translation;
    use crate::testing::factories;

    #[test]
    fn allows_multi_square_move_forward_white() {
        let from_square = Square::new(Rank::ONE, File::A);
        let to_square = Square::new(Rank::THREE, File::A);
        let piece = Piece::new(Colour::White, PieceType::Rook);
        let move_ = Move::new(&piece, &from_square, &to_square);

        let rule = MultiSquareMove::new(translation::ChessVector::new(0, 1));

        assert!(rule.allows_move(&move_));
    }

    #[test]
    fn allows_multi_square_move_forward_back() {
        let from_square = Square::new(Rank::FIVE, File::D);
        let to_square = Square::new(Rank::THREE, File::D);
        let piece = Piece::new(Colour::Black, PieceType::Rook);
        let move_ = Move::new(&piece, &from_square, &to_square);

        let rule = MultiSquareMove::new(translation::ChessVector::new(0, 1));

        assert!(rule.allows_move(&move_));
    }

    #[test]
    fn disallows_move_in_wrong_direction() {
        let from_square = Square::new(Rank::ONE, File::A);
        let to_square = Square::new(Rank::THREE, File::C);
        let piece = factories::some_piece();
        let move_ = Move::new(&piece, &from_square, &to_square);

        let vector = translation::ChessVector::new(0, -1);
        let rule = MultiSquareMove::new(vector);

        assert!(!rule.allows_move(&move_));
    }
}
