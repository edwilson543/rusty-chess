use super::{ordinary_move, translation};

use super::base_move;
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
    previous_move: ordinary_move::OrdinaryMove,
    translation: translation::Translation,
}

impl EnPassant {
    pub fn new(
        pawn: &chess_set::Piece,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
        previous_move: &ordinary_move::OrdinaryMove,
    ) -> Self {
        let translation =
            translation::Translation::from_move(from_square, to_square, pawn.get_colour());

        Self {
            pawn: pawn.clone(),
            from_square: from_square.clone(),
            to_square: to_square.clone(),
            previous_move: previous_move.clone(),
            translation: translation,
        }
    }

    pub fn validate(
        &self,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> Result<(), EnPassantValidationError> {
        if !(self.pawn.get_piece_type() == &chess_set::PieceType::Pawn) {
            return Err(EnPassantValidationError::OnlyAllowedForPawns);
        }
        if !is_double_pawn_advancement(&self.previous_move) {
            return Err(EnPassantValidationError::OnlyAllowedAfterDoubleAdvancement);
        }

        if !target_square_is_valid(&self, &self.previous_move) {
            return Err(EnPassantValidationError::InvalidTargetSquare);
        };

        return Ok(());
    }
}

impl base_move::ChessMove for EnPassant {
    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError> {
        match chessboard.remove_piece(&self.previous_move.to_square) {
            Err(error) => return Err(error),
            Ok(_) => {}
        };

        chessboard.move_piece(&self.from_square, &self.to_square)
    }
}

// En passant is only allowed immediately after the opponent makes a double pawn advancement.
// fn did_opponent_pawn_just_jump_over_target_square(
//     to_square: &chess_set::Square,
//     chessboard_history: &Vec<chess_set::Chessboard>,
// ) -> bool {
//     // Todo -> Do chessboard_history[-1] - chessboard_history[-2].
// - The diff should contain two items
// - Check the piece in both cases is the same pawn
// - Check the squares are two apart (so can delete `apply_to_square`)
// }

// TODO -> old.
fn is_double_pawn_advancement(previous_move: &ordinary_move::OrdinaryMove) -> bool {
    let was_pawn = previous_move.piece.get_piece_type() == &chess_set::PieceType::Pawn;
    // Pawns can only move two squares if it is forwards, so no need to check direction.
    let was_double_advancement = previous_move.translation.scalar == 2;
    was_double_advancement && was_pawn
}

// En passant is only allowed to the middle square of a double pawn advancement.
fn target_square_is_valid(
    en_passant: &EnPassant,
    previous_move: &ordinary_move::OrdinaryMove,
) -> bool {
    let forwards_and_right = translation::ChessVector::new(1, 1);
    let forwards_and_left = translation::ChessVector::new(-1, 1);

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
