use super::super::rule::OrdinaryMoveRule;
use crate::domain::gameplay::rulebook::moves::ordinary_move;
use crate::domain::gameplay::rulebook::moves::translation;
use std::cmp;

pub struct MultiSquareMove {
    vector: translation::ChessVector,
}

impl MultiSquareMove {
    pub fn new(vector: translation::ChessVector) -> Self {
        Self { vector }
    }
}

impl OrdinaryMoveRule for MultiSquareMove {
    fn allows_move(&self, chess_move: &ordinary_move::OrdinaryMove) -> bool {
        let translation = &chess_move.translation;
        self.vector == translation.vector && !self.is_obstructed(chess_move)
    }
}

impl MultiSquareMove {
    pub fn is_obstructed(&self, chess_move: &ordinary_move::OrdinaryMove) -> bool {
        false
        // TODO.
        // if !self.is_straight_line() {
        //     return true
        // }
        //
        // for scalar in 1..=chess_move.translation.scalar {
        //     // TODO -> can't do this, since vector is in opposite direction for white / black.
        //     // TODO -> need to add a "direction" field to translation.
        //     let rank = chess_move.from_square.get_rank().index() + self.vector.y;
        // }
        //
        // return false;
    }

    fn is_straight_line(&self) -> bool {
        // Straight line => plus / diags.
        // i.e. anything where {x, y} < {-1, 0, 1}
        cmp::max(self.vector.x.abs(), self.vector.y.abs()) <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::ordinary_move::OrdinaryMove;
    use crate::domain::gameplay::rulebook::moves::translation;
    use crate::testing::factories;

    #[test]
    fn allows_multi_square_move_forward_white() {
        let from_square = Square::new(Rank::One, File::A);
        let to_square = Square::new(Rank::Three, File::A);
        let piece = Piece::new(Colour::White, PieceType::Rook);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &piece, &from_square, &to_square);

        let rule = MultiSquareMove::new(translation::ChessVector::new(0, 1));

        assert!(rule.allows_move(&chess_move));
    }

    #[test]
    fn allows_multi_square_move_forward_back() {
        let from_square = Square::new(Rank::Five, File::D);
        let to_square = Square::new(Rank::Three, File::D);
        let piece = Piece::new(Colour::Black, PieceType::Rook);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &piece, &from_square, &to_square);

        let rule = MultiSquareMove::new(translation::ChessVector::new(0, -1));

        assert!(rule.allows_move(&chess_move));
    }

    #[test]
    fn disallows_move_in_wrong_direction() {
        let from_square = Square::new(Rank::One, File::A);
        let to_square = Square::new(Rank::Three, File::C);
        let piece = factories::some_piece();

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &piece, &from_square, &to_square);

        let vector = translation::ChessVector::new(0, -1);
        let rule = MultiSquareMove::new(vector);

        assert!(!rule.allows_move(&chess_move));
    }
}
