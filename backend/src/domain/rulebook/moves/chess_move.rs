use super::{pieces, translation};
use crate::domain::chess_set;
use std::collections::BTreeMap;
use std::fmt;
use thiserror;

/// Enumeration of all errors that can be raised when validating chess moves.
///
/// These are defined centrally rather than generically or by association,
/// to allow passing `Move`s around dynamically.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MoveValidationError {
    // Ordinary moves.
    PieceIsNotAtFromSquare,
    MoveIsNotLegalForPiece,
    CannotCaptureOwnPiece,
    CannotCaptureOpponentKing,
}

pub struct Move {
    pub piece: chess_set::Piece,
    pub from_square: chess_set::Square,
    pub to_square: chess_set::Square,
    pub translation: translation::Translation,
}

pub trait MoveRule {
    /// Test whether a certain move is valid, according to this rule.
    fn allows_move(
        &self,
        chess_move: &Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool;

    /// Return the outcome of playing a valid move, according to this rule.
    ///
    /// Note that special rules must override this method. For example:
    /// * "en passant" involves capturing the piece at a square different to `to_square`
    /// * "castling" involves moving two pieces
    fn get_move_outcome(
        &self,
        chess_move: &Move,
    ) -> BTreeMap<chess_set::Square, Option<chess_set::Piece>> {
        let mut outcome = BTreeMap::new();
        outcome.insert(chess_move.from_square, None);
        outcome.insert(chess_move.to_square, Some(chess_move.piece));

        outcome
    }
}

impl Move {
    // Factories.
    pub fn new(
        piece: chess_set::Piece,
        from_square: chess_set::Square,
        to_square: chess_set::Square,
    ) -> Self {
        let translation = translation::Translation::from_move(&from_square, &to_square);

        Self {
            piece: piece,
            from_square: from_square,
            to_square: to_square,
            translation: translation,
        }
    }

    pub fn apply_if_valid(
        &self,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> Result<chess_set::Chessboard, MoveValidationError> {
        let allowing_rule = match self.validate(chessboard_history) {
            Ok(rule) => rule,
            Err(error) => return Err(error),
        };

        let mut chessboard = chessboard_history.last().unwrap().clone();
        let move_outcome = allowing_rule.get_move_outcome(self);
        chessboard.update_position(move_outcome);
        Ok(chessboard)
    }

    // Queries.
    pub fn validate(
        &self,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> Result<Box<dyn MoveRule>, MoveValidationError> {
        let chessboard = chessboard_history.last().unwrap();

        let rule = match self.get_rule_that_allows_move(chessboard_history) {
            Ok(rule) => rule,
            Err(error) => return Err(error),
        };

        if !(chessboard.get_piece(&self.from_square) == Some(self.piece)) {
            return Err(MoveValidationError::PieceIsNotAtFromSquare);
        }

        if let Err(error) = self.validate_occupant_of_to_square(chessboard) {
            return Err(error);
        };

        Ok(rule)
    }

    pub fn is_obstructed(&self, chessboard: &chess_set::Chessboard) -> bool {
        if !self.translation.vector.is_straight_line() {
            return true;
        }

        for scalar in 1..self.translation.scalar {
            let rank_index =
                self.from_square.get_rank().index() + self.translation.vector.y * (scalar as i8);
            let file_index =
                self.from_square.get_file().index() + self.translation.vector.x * (scalar as i8);
            let square = chess_set::Square::from_indexes(rank_index, file_index);
            if let Some(_) = chessboard.get_piece(&square) {
                return true;
            }
        }

        return false;
    }

    fn get_rule_that_allows_move(
        &self,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> Result<Box<dyn MoveRule>, MoveValidationError> {
        let rules = pieces::get_move_rules_for_piece(self.piece.get_piece_type());
        for rule in rules {
            if rule.allows_move(self, chessboard_history) {
                return Ok(rule);
            }
        }
        Err(MoveValidationError::MoveIsNotLegalForPiece)
    }

    fn validate_occupant_of_to_square(
        &self,
        chessboard: &chess_set::Chessboard,
    ) -> Result<(), MoveValidationError> {
        let Some(opponent_piece) = chessboard.get_piece(&self.to_square) else {
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
    #[cfg(test)]
    mod test_validate {
        use super::super::*;
        use crate::domain::chess_set;
        use crate::testing::factories;
        use std::collections::BTreeMap;

        #[test]
        fn can_move_piece_to_empty_square() {
            let chessboard = factories::chessboard();
            let from_square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let to_square = chess_set::Square::new(chess_set::Rank::Three, chess_set::File::A);
            let piece = chessboard.get_piece(&from_square).unwrap();

            let chess_move = Move::new(piece, from_square, to_square);

            let result = chess_move.validate(&vec![chessboard]);

            assert!(result.is_ok());
        }

        #[test]
        fn cannot_move_piece_to_same_square() {
            let chessboard = factories::chessboard();
            let square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let piece = chessboard.get_piece(&square).unwrap();
            let chess_move = Move::new(piece, square.clone(), square);

            let result = chess_move.validate(&vec![chessboard]);

            let expected_error = MoveValidationError::MoveIsNotLegalForPiece;
            match result {
                Err(error) => assert_eq!(error, expected_error),
                Ok(_) => assert!(false),
            }
        }

        #[test]
        fn cannot_move_piece_to_square_occupied_by_another_piece_of_the_same_colour() {
            let mut starting_position = BTreeMap::new();
            let piece = chess_set::Piece::new(chess_set::Colour::White, chess_set::PieceType::Rook);
            let from_square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            starting_position.insert(from_square, piece);

            let to_square = chess_set::Square::new(chess_set::Rank::Three, chess_set::File::A);
            let other_piece =
                chess_set::Piece::new(chess_set::Colour::White, chess_set::PieceType::Bishop);
            starting_position.insert(to_square, other_piece);

            let chessboard = chess_set::Chessboard::new(starting_position);

            let chess_move = Move::new(piece, from_square, to_square);
            let result = chess_move.validate(&vec![chessboard]);

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
            let _ = chessboard.position.insert(to_square, Some(black_king));

            let chess_move = Move::new(white_pawn, from_square, to_square);
            let result = chess_move.validate(&vec![chessboard]);

            let expected_error = MoveValidationError::CannotCaptureOpponentKing;
            match result {
                Err(error) => assert_eq!(error, expected_error),
                Ok(_) => assert!(false),
            }
        }
    }

    #[cfg(test)]
    mod test_apply_if_valid {
        use super::super::*;
        use crate::domain::chess_set;
        use crate::testing::factories;

        #[test]
        fn can_move_piece_to_empty_square() {
            let chessboard = factories::chessboard();
            let from_square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let to_square = chess_set::Square::new(chess_set::Rank::Three, chess_set::File::A);
            let piece = chessboard.get_piece(&from_square).unwrap();

            let chess_move = Move::new(piece, from_square.clone(), to_square.clone());

            let result = chess_move.apply_if_valid(&vec![chessboard]);

            let updated_chessboard = result.unwrap();
            assert_eq!(
                updated_chessboard.position.get(&from_square).unwrap(),
                &None
            );
            assert_eq!(
                updated_chessboard.position.get(&to_square).unwrap(),
                &Some(piece)
            );
        }

        #[test]
        fn cannot_move_piece_to_same_square() {
            let chessboard = factories::chessboard();
            let square = chess_set::Square::new(chess_set::Rank::Two, chess_set::File::A);
            let piece = chessboard.get_piece(&square).unwrap();
            let chess_move = Move::new(piece, square.clone(), square);

            let result = chess_move.apply_if_valid(&vec![chessboard]);

            let expected_error = MoveValidationError::MoveIsNotLegalForPiece;
            assert_eq!(result, Err(expected_error))
        }
    }
}
