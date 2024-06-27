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

    // Check square can be moved to.

    // Check translation is allowed for piece.
    // Check translation is unobstructed (if not a knight).
    // Check would not be left in check.
    Ok(())
}

fn can_square_be_moved_to(
    chessboard: &chess_set::Chessboard,
    piece: &chess_set::Piece,
    to_square: &chess_set::Square,
) -> Result<(), MoveValidationError> {
    let Some(opponent_piece) = chessboard.get_piece(&to_square) else {
        return Ok(());
    };

    if piece.get_colour() == opponent_piece.get_colour() {
        return Err(PlyValidationError::CannotCaptureOwnPiece);
    };

    if opponent_piece.get_piece_type() == &chess_set::PieceType::King {
        return Err(MoveValidationError::CannotCaptureOpponentKing);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn cannot_move_piece_to_same_square() {}
}
