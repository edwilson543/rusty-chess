use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook::moves::{ordinary_move, pieces};
use std::fmt;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MoveValidationError {
    CannotMovePieceToSameSquare,
    CannotCaptureOwnPiece,
    CannotCaptureOpponentKing,
    MoveIsNotLegalForPiece,
}

pub fn validate_move(
    chessboard: &chess_set::Chessboard,
    piece: &chess_set::Piece,
    from_square: &chess_set::Square,
    to_square: &chess_set::Square,
) -> Result<ordinary_move::OrdinaryMove, MoveValidationError> {
    if from_square == to_square {
        return Err(MoveValidationError::CannotMovePieceToSameSquare);
    };

    if let Err(error) = validate_occupant_of_target_square(chessboard, piece, to_square) {
        return Err(error);
    }

    let chess_move = ordinary_move::OrdinaryMove::new(chessboard, piece, from_square, to_square);
    if let Err(error) = validate_move_is_legal(&chess_move) {
        return Err(error);
    }

    Ok(chess_move)
}

fn validate_occupant_of_target_square(
    chessboard: &chess_set::Chessboard,
    piece: &chess_set::Piece,
    to_square: &chess_set::Square,
) -> Result<(), MoveValidationError> {
    let Some(opponent_piece) = chessboard.get_piece(to_square) else {
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

fn validate_move_is_legal(
    chess_move: &ordinary_move::OrdinaryMove,
) -> Result<(), MoveValidationError> {
    let piece_type = chess_move.piece.get_piece_type();
    let mut move_rules = pieces::get_rules_for_piece(piece_type);

    let permitted_by_translation_rules = move_rules
        .any(|rule: Box<dyn ordinary_move::OrdinaryMoveRule>| rule.allows_move(&chess_move));

    match permitted_by_translation_rules {
        true => Ok(()),
        false => Err(MoveValidationError::MoveIsNotLegalForPiece),
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
        let from_square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
        let to_square = chess_set::Square::new(chess_set::Rank::Three, chess_set::File::A);
        let piece = chessboard.get_piece(&from_square).unwrap();

        let result = validate_move(&chessboard, &piece, &from_square, &to_square);

        assert!(result.is_ok());
    }

    #[cfg(test)]
    mod test_can_square_be_moved_to {
        use super::super::*;
        use crate::domain::gameplay::chess_set;
        use crate::testing::factories;

        #[test]
        fn cannot_move_piece_to_same_square() {
            let chessboard = factories::chessboard();
            let square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let piece = chessboard.get_piece(&square).unwrap();

            let result = validate_move(&chessboard, &piece, &square, &square);

            let expected_error = MoveValidationError::CannotMovePieceToSameSquare;
            match result {
                Err(error) => assert_eq!(error, expected_error),
                Ok(_) => assert!(false),
            }
        }

        #[test]
        fn cannot_move_piece_to_square_occupied_by_another_piece_of_the_same_colour() {
            let mut chessboard = factories::chessboard();
            let from_square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let piece = chessboard.get_piece(&from_square).unwrap();

            let to_square = chess_set::Square::new(chess_set::Rank::Three, chess_set::File::A);
            let other_piece =
                chess_set::Piece::new(piece.get_colour().clone(), chess_set::PieceType::Pawn);
            let _ = chessboard.add_piece(other_piece, &to_square);

            let result = validate_move(&chessboard, &piece, &from_square, &to_square);

            let expected_error = MoveValidationError::CannotCaptureOwnPiece;
            match result {
                Err(error) => assert_eq!(error, expected_error),
                Ok(_) => assert!(false),
            }
        }

        #[test]
        fn cannot_capture_opponent_king() {
            let mut chessboard = factories::chessboard();
            let from_square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let white_pawn = chessboard.get_piece(&from_square).unwrap();

            let to_square = chess_set::Square::new(chess_set::Rank::Three, chess_set::File::B);
            let black_king =
                chess_set::Piece::new(chess_set::Colour::Black, chess_set::PieceType::King);
            let _ = chessboard.add_piece(black_king, &to_square);

            let result = validate_move(&chessboard, &white_pawn, &from_square, &to_square);

            let expected_error = MoveValidationError::CannotCaptureOpponentKing;
            match result {
                Err(error) => assert_eq!(error, expected_error),
                Ok(_) => assert!(false),
            }
        }
    }
}
