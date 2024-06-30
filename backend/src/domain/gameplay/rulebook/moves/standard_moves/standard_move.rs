use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook::moves::translations;
use std::fmt;

/// A standard move of a single piece from one square to another.
pub struct Move<'a> {
    piece: &'a chess_set::Piece,
    from_square: &'a chess_set::Square,
    to_square: &'a chess_set::Square,
    translation: translations::Translation,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MoveValidationError {
    CannotMovePieceToSameSquare,
    CannotCaptureOwnPiece,
    CannotCaptureOpponentKing,
}

impl<'a> Move<'a> {
    pub fn new(
        piece: &'a chess_set::Piece,
        from_square: &'a chess_set::Square,
        to_square: &'a chess_set::Square,
    ) -> Self {
        let translation =
            translations::Translation::new(from_square, to_square, piece.get_colour());

        Self {
            piece: piece,
            from_square: from_square,
            to_square: to_square,
            translation: translation,
        }
    }

    pub fn validate(&self, chessboard: &chess_set::Chessboard) -> Result<(), MoveValidationError> {
        if self.from_square == self.to_square {
            return Err(MoveValidationError::CannotMovePieceToSameSquare);
        };

        if let Err(error) = self.validate_occupant_of_target_square(chessboard) {
            return Err(error);
        }

        // TODO -> validate translation.
        Ok(())
    }

    fn validate_occupant_of_target_square(
        &self,
        chessboard: &chess_set::Chessboard,
    ) -> Result<(), MoveValidationError> {
        let Some(opponent_piece) = chessboard.get_piece(self.to_square) else {
            return Ok(());
        };

        if opponent_piece.get_colour() == self.piece.get_colour() {
            return Err(MoveValidationError::CannotCaptureOwnPiece);
        };

        if opponent_piece.get_piece_type() == &chess_set::PieceType::King {
            return Err(MoveValidationError::CannotCaptureOpponentKing);
        };

        Ok(())
    }
}

// Trait implementations.

impl fmt::Display for MoveValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::factories;

    #[test]
    fn can_move_piece_to_empty_square() {
        let chessboard = factories::chessboard();
        let from_square = chess_set::Square::new(chess_set::Rank::TWO, chess_set::File::A);
        let to_square = chess_set::Square::new(chess_set::Rank::THREE, chess_set::File::A);
        let piece = chessboard.get_piece(&from_square).unwrap();
        let move_ = Move::new(&piece, &from_square, &to_square);

        let result = move_.validate(&chessboard);

        assert_eq!(result, Ok(()));
    }

    #[cfg(test)]
    mod test_can_square_be_moved_to {
        use super::super::*;
        use crate::domain::gameplay::chess_set;
        use crate::testing::factories;

        #[test]
        fn cannot_move_piece_to_same_square() {
            let chessboard = factories::chessboard();
            let square = chess_set::Square::new(chess_set::Rank::TWO, chess_set::File::A);
            let piece = chessboard.get_piece(&square).unwrap();
            let move_ = Move::new(&piece, &square, &square);

            let result = move_.validate(&chessboard);

            assert_eq!(
                result,
                Err(MoveValidationError::CannotMovePieceToSameSquare)
            );
        }

        #[test]
        fn cannot_move_piece_to_square_occupied_by_another_piece_of_the_same_colour() {
            let mut chessboard = factories::chessboard();
            let from_square = chess_set::Square::new(chess_set::Rank::TWO, chess_set::File::A);
            let piece = chessboard.get_piece(&from_square).unwrap();

            let to_square = chess_set::Square::new(chess_set::Rank::THREE, chess_set::File::A);
            let other_piece =
                chess_set::Piece::new(piece.get_colour().clone(), chess_set::PieceType::Pawn);
            let _ = chessboard.add_piece(other_piece, &to_square);

            let move_ = Move::new(&piece, &from_square, &to_square);

            let result = move_.validate(&chessboard);

            assert_eq!(result, Err(MoveValidationError::CannotCaptureOwnPiece));
        }

        #[test]
        fn cannot_capture_opponent_king() {
            let mut chessboard = factories::chessboard();
            let from_square = chess_set::Square::new(chess_set::Rank::TWO, chess_set::File::A);
            let white_pawn = chessboard.get_piece(&from_square).unwrap();

            let to_square = chess_set::Square::new(chess_set::Rank::THREE, chess_set::File::B);
            let black_king =
                chess_set::Piece::new(chess_set::Colour::Black, chess_set::PieceType::King);
            let _ = chessboard.add_piece(black_king, &to_square);

            let move_ = Move::new(&white_pawn, &from_square, &to_square);

            let result = move_.validate(&chessboard);

            assert_eq!(result, Err(MoveValidationError::CannotCaptureOpponentKing));
        }
    }
}
