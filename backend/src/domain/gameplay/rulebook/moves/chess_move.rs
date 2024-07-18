use crate::domain::gameplay::chess_set;
use std::fmt;

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

    // En passant.
    EnPassantOnlyAllowedForPawns,
    EnPassantOnlyAllowedAfterDoubleAdvancement,
    EnPassantInvalidTranslation,
}

/// Any move that can be played in chess.
pub trait Move {
    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError>;

    fn validate(
        &self,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> Result<(), MoveValidationError>;
}

// Trait implementations.

impl fmt::Display for MoveValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
