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
        if !self.did_opponent_pawn_make_double_pawn_advancement(chessboard_history) {
            return Err(EnPassantValidationError::OnlyAllowedAfterDoubleAdvancement);
        }

        if !target_square_is_valid(&self, &self.previous_move) {
            return Err(EnPassantValidationError::InvalidTargetSquare);
        };

        return Ok(());
    }

    // En passant is only allowed immediately after the opponent makes a double pawn advancement.
    fn did_opponent_pawn_make_double_pawn_advancement(
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
