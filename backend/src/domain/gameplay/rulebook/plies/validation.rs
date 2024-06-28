use crate::domain::gameplay::chess_set;
use std::fmt;
use std::fmt::Formatter;
use thiserror;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MoveValidationError {
    CannotMovePieceToSameSquare,
    CannotCaptureOwnPiece,
    CannotCaptureOpponentKing,
}

impl fmt::Display for MoveValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Validate whether a given move is in compliance with the chess rules.
pub fn validate_move(
    chessboard: &chess_set::Chessboard,
    piece: &chess_set::Piece,
    from_square: &chess_set::Square,
    to_square: &chess_set::Square,
) -> Result<(), MoveValidationError> {
    if from_square == to_square {
        return Err(MoveValidationError::CannotMovePieceToSameSquare);
    };

    if let Err(error) = validate_occupant_of_target_square(chessboard, piece, to_square) {
        return Err(error);
    }

    // Check translation is allowed for piece.
    // Check translation is unobstructed (if not a knight).
    // Check would not be left in check.
    Ok(())
}

fn validate_occupant_of_target_square(
    chessboard: &chess_set::Chessboard,
    piece: &chess_set::Piece,
    to_square: &chess_set::Square,
) -> Result<(), MoveValidationError> {
    let Some(opponent_piece) = chessboard.get_piece(&to_square) else {
        return Ok(());
    };

    if opponent_piece.get_colour() == piece.get_colour() {
        return Err(MoveValidationError::CannotCaptureOwnPiece);
    };

    if opponent_piece.get_piece_type() == &chess_set::PieceType::King {
        return Err(MoveValidationError::CannotCaptureOpponentKing);
    };

    Ok(())
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

        let result = validate_move(&chessboard, &piece, &from_square, &to_square);

        assert_eq!(result, Ok(()));
    }

    #[cfg(test)]
    mod test_can_square_be_moved_to {
        use crate::domain::gameplay::chess_set;
        use crate::domain::gameplay::rulebook::{validate_move, MoveValidationError};
        use crate::testing::factories;

        #[test]
        fn cannot_move_piece_to_same_square() {
            let chessboard = factories::chessboard();
            let square = chess_set::Square::new(chess_set::Rank::TWO, chess_set::File::A);
            let piece = chessboard.get_piece(&square).unwrap();

            let result = validate_move(&chessboard, &piece, &square, &square);

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
            chessboard.add_piece(other_piece, &to_square);

            let result = validate_move(&chessboard, &piece, &from_square, &to_square);

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
            chessboard.add_piece(black_king, &to_square);

            let result = validate_move(&chessboard, &white_pawn, &from_square, &to_square);

            assert_eq!(result, Err(MoveValidationError::CannotCaptureOpponentKing));
        }
    }
}
