use super::translation;
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
    CannotMovePieceToSameSquare,
    CannotCaptureOwnPiece,
    CannotCaptureOpponentKing,
    MoveIsNotLegalForPiece,
}

pub struct Move {
    pub piece: chess_set::Piece,
    pub from_square: chess_set::Square,
    pub to_square: chess_set::Square,
    pub translation: translation::Translation,
}

pub struct MoveOutcome {
    pub outcome: BTreeMap<chess_set::Square, Option<chess_set::Piece>>,
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

    // Queries.
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
}

// Trait implementations.

impl fmt::Display for MoveValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
