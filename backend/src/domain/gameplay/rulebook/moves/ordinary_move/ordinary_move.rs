use super::super::{base_move, translation};
use super::{pieces, rule};
use crate::domain::gameplay::chess_set;
use std::fmt;

/// A move of a single piece from one square to another.
#[derive(Clone)]
pub struct OrdinaryMove {
    pub chessboard: chess_set::Chessboard,
    pub piece: chess_set::Piece,
    pub from_square: chess_set::Square,
    pub to_square: chess_set::Square,
    pub translation: translation::Translation,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MoveValidationError {
    CannotMovePieceToSameSquare,
    CannotCaptureOwnPiece,
    CannotCaptureOpponentKing,
    MoveIsNotLegalForPiece,
}

impl base_move::ChessMove for OrdinaryMove {
    type Error = MoveValidationError;

    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError> {
        chessboard.move_piece(&self.from_square, &self.to_square)
    }

    fn validate(
        &self,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> Result<(), MoveValidationError> {
        let _ = chessboard_history; // To avoid a catch-all warning.

        if self.from_square == self.to_square {
            return Err(MoveValidationError::CannotMovePieceToSameSquare);
        };

        if let Err(error) = self.validate_move_is_legal() {
            return Err(error);
        }

        if let Err(error) = self.validate_occupant_of_target_square() {
            return Err(error);
        }

        Ok(())
    }
}

impl OrdinaryMove {
    pub fn new(
        chessboard: &chess_set::Chessboard,
        piece: &chess_set::Piece,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Self {
        let translation = translation::Translation::from_move(from_square, to_square);

        Self {
            chessboard: chessboard.clone(),
            piece: piece.clone(),
            from_square: from_square.clone(),
            to_square: to_square.clone(),
            translation: translation,
        }
    }
}

impl OrdinaryMove {
    fn validate_move_is_legal(&self) -> Result<(), MoveValidationError> {
        let piece_type = self.piece.get_piece_type();
        let mut move_rules = pieces::get_rules_for_piece(piece_type);

        let permitted_by_translation_rules =
            move_rules.any(|rule: Box<dyn rule::OrdinaryMoveRule>| rule.allows_move(&self));

        match permitted_by_translation_rules {
            true => Ok(()),
            false => Err(MoveValidationError::MoveIsNotLegalForPiece),
        }
    }

    fn validate_occupant_of_target_square(&self) -> Result<(), MoveValidationError> {
        let Some(opponent_piece) = self.chessboard.get_piece(&self.to_square) else {
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
    use crate::domain::gameplay::rulebook::ChessMove;
    use crate::testing::factories;

    #[test]
    fn can_move_piece_to_empty_square() {
        let chessboard = factories::chessboard();
        let from_square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
        let to_square = chess_set::Square::new(chess_set::Rank::Three, chess_set::File::A);
        let piece = chessboard.get_piece(&from_square).unwrap();

        let ordinary_move = OrdinaryMove::new(&chessboard, &piece, &from_square, &to_square);

        let result = ordinary_move.validate(&vec![chessboard]);

        assert!(result.is_ok());
    }

    #[cfg(test)]
    mod test_can_square_be_moved_to {
        use super::super::*;
        use crate::domain::gameplay::chess_set;
        use crate::domain::gameplay::rulebook::ChessMove;
        use crate::testing::factories;

        #[test]
        fn cannot_move_piece_to_same_square() {
            let chessboard = factories::chessboard();
            let square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let piece = chessboard.get_piece(&square).unwrap();
            let ordinary_move = OrdinaryMove::new(&chessboard, &piece, &square, &square);

            let result = ordinary_move.validate(&vec![chessboard]);

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

            let ordinary_move = OrdinaryMove::new(&chessboard, &piece, &from_square, &to_square);
            let result = ordinary_move.validate(&vec![chessboard]);

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

            let ordinary_move =
                OrdinaryMove::new(&chessboard, &white_pawn, &from_square, &to_square);
            let result = ordinary_move.validate(&vec![chessboard]);

            let expected_error = MoveValidationError::CannotCaptureOpponentKing;
            match result {
                Err(error) => assert_eq!(error, expected_error),
                Ok(_) => assert!(false),
            }
        }
    }
}
