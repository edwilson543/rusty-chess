use super::ordinary_move::OrdinaryMove;
use super::rule::OrdinaryMoveRule;
use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook::moves::translation;

pub struct MultiSquareMove {
    vector: translation::ChessVector,
}

impl MultiSquareMove {
    pub fn new(vector: translation::ChessVector) -> Self {
        Self { vector }
    }
}

impl OrdinaryMoveRule for MultiSquareMove {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool {
        let translation = &chess_move.translation;
        self.vector == translation.vector && !self.is_obstructed(chess_move)
    }
}

impl MultiSquareMove {
    fn is_obstructed(&self, chess_move: &OrdinaryMove) -> bool {
        if !chess_move.translation.vector.is_straight_line() {
            return true;
        }

        for scalar in 1..chess_move.translation.scalar {
            let rank_index =
                chess_move.from_square.get_rank().index() + self.vector.y * (scalar as i8);
            let file_index =
                chess_move.from_square.get_file().index() + self.vector.x * (scalar as i8);
            let square = chess_set::Square::from_indexes(rank_index, file_index);
            if let Some(_) = chess_move.chessboard.get_piece(&square) {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{
        Chessboard, Colour, File, Piece, PieceType, Rank, Square,
    };
    use crate::domain::gameplay::rulebook::moves::ordinary_move::OrdinaryMove;
    use crate::domain::gameplay::rulebook::moves::translation;
    use crate::testing::factories;
    use std::collections::BTreeMap;

    #[test]
    fn allows_multi_square_move_forward_white() {
        let from_square = Square::new(Rank::Two, File::A);
        let to_square = Square::new(Rank::Four, File::A);
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
    fn allows_move_to_capture_opponent_piece() {
        let from_square = Square::new(Rank::Five, File::D);
        let to_square = Square::new(Rank::Three, File::D);
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);

        let mut starting_position = BTreeMap::new();
        starting_position.insert(from_square, black_rook);
        starting_position.insert(to_square, white_pawn);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = OrdinaryMove::new(&chessboard, &black_rook, &from_square, &to_square);

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

    #[test]
    fn cannot_move_white_rook_from_back_rank_when_obstructed_by_pawn() {
        let from_square = Square::new(Rank::One, File::H);
        let to_square = Square::new(Rank::Four, File::H);
        let piece = Piece::new(Colour::White, PieceType::Rook);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &piece, &from_square, &to_square);

        let rule = MultiSquareMove::new(translation::ChessVector::new(0, 1));

        assert!(!rule.allows_move(&chess_move));
    }

    #[test]
    fn cannot_move_black_bishop_from_back_rank_when_obstructed_by_pawn() {
        let from_square = Square::new(Rank::Eight, File::C);
        let to_square = Square::new(Rank::Six, File::E);
        let piece = Piece::new(Colour::Black, PieceType::Bishop);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &piece, &from_square, &to_square);

        let rule = MultiSquareMove::new(translation::ChessVector::new(0, 1));

        assert!(!rule.allows_move(&chess_move));
    }
}
