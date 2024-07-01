use super::moves;

use crate::domain::gameplay::chess_set;
use std::fmt;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum EnPassantValidationError {
    OnlyAllowedForPawns,
    OnlyAllowedAfterDoubleAdvancement,
    InvalidTargetSquare,
}

pub struct EnPassant {
    pawn: chess_set::Piece,
    from_square: chess_set::Square,
    to_square: chess_set::Square,
    translation: moves::Translation,
}

pub fn validate_en_passant(
    piece: &chess_set::Piece,
    from_square: &chess_set::Square,
    to_square: &chess_set::Square,
    previous_move: &moves::Move,
) -> Result<EnPassant, EnPassantValidationError> {
    if !(piece.get_piece_type() == &chess_set::PieceType::Pawn) {
        return Err(EnPassantValidationError::OnlyAllowedForPawns);
    }
    if !is_double_pawn_advancement(previous_move) {
        return Err(EnPassantValidationError::OnlyAllowedAfterDoubleAdvancement);
    }

    let en_passant = EnPassant::new(piece, from_square, to_square);
    if !target_square_is_valid(&en_passant, previous_move) {
        return Err(EnPassantValidationError::InvalidTargetSquare);
    };

    return Ok(en_passant);
}

impl EnPassant {
    pub fn new(
        pawn: &chess_set::Piece,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Self {
        let translation = moves::Translation::from_move(from_square, to_square, pawn.get_colour());

        Self {
            pawn: pawn.clone(),
            from_square: from_square.clone(),
            to_square: to_square.clone(),
            translation: translation,
        }
    }
}

// En passant is only allowed immediately after the opponent makes a double pawn advancement.
fn is_double_pawn_advancement(previous_move: &moves::Move) -> bool {
    let was_pawn = previous_move.piece.get_piece_type() == &chess_set::PieceType::Pawn;
    // Pawns can only move two squares if it is forwards, so no need to check direction.
    let was_double_advancement = previous_move.translation.scalar == 2;
    was_double_advancement && was_pawn
}

// En passant is only allowed to the middle square of a double pawn advancement.
fn target_square_is_valid(en_passant: &EnPassant, previous_move: &moves::Move) -> bool {
    let forwards_and_right = moves::ChessVector::new(1, 1);
    let forwards_and_left = moves::ChessVector::new(-1, 1);

    let move_is_diagonal = en_passant.translation.vector == forwards_and_right
        || en_passant.translation.vector == forwards_and_left;

    let is_to_correct_file = en_passant.to_square.get_file() == previous_move.to_square.get_file();
    move_is_diagonal && en_passant.translation.scalar == 1 && is_to_correct_file
}

// Trait implementations.

impl fmt::Display for EnPassantValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// TODO -> add unit tests for error paths.
