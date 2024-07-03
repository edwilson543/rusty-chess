use super::translation;

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
    translation: translation::Translation,
}

impl base_move::ChessMove for EnPassant {
    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError> {
        let square = self.get_square_captured_pawn_should_have_moved_to();
        match chessboard.remove_piece(&square) {
            Err(error) => return Err(error),
            Ok(_) => {}
        };

        chessboard.move_piece(&self.from_square, &self.to_square)
    }
}

impl EnPassant {
    pub fn new(
        pawn: &chess_set::Piece,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Self {
        let translation =
            translation::Translation::from_move(from_square, to_square, pawn.get_colour());

        Self {
            pawn: pawn.clone(),
            from_square: from_square.clone(),
            to_square: to_square.clone(),
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
        if !self.opponent_made_double_pawn_advancement_over_target_square(chessboard_history) {
            return Err(EnPassantValidationError::OnlyAllowedAfterDoubleAdvancement);
        }

        if !self.is_translation_valid() {
            return Err(EnPassantValidationError::InvalidTargetSquare);
        };

        return Ok(());
    }

    // En passant is only allowed immediately after the opponent makes a double pawn advancement.
    fn opponent_made_double_pawn_advancement_over_target_square(
        &self,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let from_square = self.get_square_captured_pawn_should_have_moved_from();
        let previous_state = &chessboard_history[chessboard_history.len() - 2];
        let opponent_pawn_moved_from_expected_square =
            self.is_piece_at_square_opponent_pawn(previous_state, &from_square);

        let current_state = chessboard_history.last().unwrap();
        let to_square = self.get_square_captured_pawn_should_have_moved_to();
        let opponent_pawn_moved_to_expected_square =
            self.is_piece_at_square_opponent_pawn(current_state, &to_square);

        opponent_pawn_moved_from_expected_square && opponent_pawn_moved_to_expected_square
    }

    fn is_piece_at_square_opponent_pawn(
        &self,
        chessboard: &chess_set::Chessboard,
        square: &chess_set::Square,
    ) -> bool {
        let Some(piece) = chessboard.get_piece(&square) else {
            return false;
        };
        let is_opponent = !(piece.get_colour() == self.pawn.get_colour());
        let is_pawn = piece.get_piece_type() == &chess_set::PieceType::Pawn;
        is_opponent && is_pawn
    }

    fn get_square_captured_pawn_should_have_moved_to(&self) -> chess_set::Square {
        let rank = self.from_square.get_rank();
        let file = self.to_square.get_file();
        chess_set::Square::new(rank.clone(), file.clone())
    }

    fn get_square_captured_pawn_should_have_moved_from(&self) -> chess_set::Square {
        let rank = match self.pawn.get_colour() {
            // Opponent pawn should have moved from their starting rank.
            &chess_set::Colour::White => chess_set::Rank::Seven,
            &chess_set::Colour::Black => chess_set::Rank::Two,
        };
        let file = self.to_square.get_file();
        chess_set::Square::new(rank.clone(), file.clone())
    }

    fn is_translation_valid(&self) -> bool {
        // An en passant must move the pawn diagonally forward one square.
        self.translation.vector == translation::ChessVector::new(1, 1)
            || self.translation.vector == translation::ChessVector::new(-1, 1)
    }
}

// Trait implementations.

impl fmt::Display for EnPassantValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// TODO -> add unit tests for error paths.
