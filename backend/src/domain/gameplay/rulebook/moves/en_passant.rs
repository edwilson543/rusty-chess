use super::translation;

use super::base_move;
use crate::domain::gameplay::chess_set;
use std::fmt;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum EnPassantValidationError {
    OnlyAllowedForPawns,
    OnlyAllowedAfterDoubleAdvancement,
    InvalidStartingSquare,
}

pub struct EnPassant {
    pawn: chess_set::Piece,
    from_square: chess_set::Square,
    to_square: chess_set::Square,
    translation: translation::Translation,
}

impl base_move::ChessMove<EnPassantValidationError> for EnPassant {
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

    fn validate(
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
            return Err(EnPassantValidationError::InvalidStartingSquare);
        };

        return Ok(());
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
        // En passant can only be made after a pawn has advanced exactly 3 squares.
        let starting_rank_valid = match self.pawn.get_colour() {
            chess_set::Colour::White => self.from_square.get_rank() == &chess_set::Rank::Five,
            chess_set::Colour::Black => self.from_square.get_rank() == &chess_set::Rank::Four,
        };

        // An en passant must move the pawn diagonally forward one square.
        let translation_valid = self.translation.vector == translation::ChessVector::new(1, 1)
            || self.translation.vector == translation::ChessVector::new(-1, 1);

        starting_rank_valid && translation_valid
    }
}

// Trait implementations.

impl fmt::Display for EnPassantValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{
        Chessboard, Colour, File, Piece, PieceType, Rank, Square,
    };
    use crate::domain::gameplay::rulebook::ChessMove;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::HashMap;

    #[rstest]
    #[case(File::C)]
    #[case(File::E)]
    fn white_can_play_en_passant(#[case] white_starting_file: File) {
        let previous_state = factories::chessboard();

        // Move the black pawn that will be captured.
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Five, File::D);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a white pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Five, white_starting_file);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        current_state.add_piece(white_pawn, &square).unwrap();

        let target_square = Square::new(Rank::Six, File::D);
        let en_passant = EnPassant::new(&white_pawn, &square, &target_square);
        let chessboard_history = vec![previous_state, current_state];

        let result = en_passant.validate(&chessboard_history);

        assert_eq!(result, Ok(()));
    }

    #[rstest]
    #[case(File::F)]
    #[case(File::H)]
    fn black_can_play_en_passant(#[case] black_starting_file: File) {
        let previous_state = factories::chessboard();

        // Move the white pawn that will be captured.
        let from_square = Square::new(Rank::Two, File::G);
        let to_square = Square::new(Rank::Four, File::G);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a black pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Four, black_starting_file);
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        current_state.add_piece(black_pawn, &square).unwrap();

        let target_square = Square::new(Rank::Three, File::G);
        let en_passant = EnPassant::new(&black_pawn, &square, &target_square);
        let chessboard_history = vec![previous_state, current_state];

        let result = en_passant.validate(&chessboard_history);

        assert_eq!(result, Ok(()));
    }

    #[rstest]
    #[case::rook(PieceType::Rook)]
    #[case::bishop(PieceType::Bishop)]
    #[case::queen(PieceType::Queen)]
    fn cannot_play_en_passant_with_a_non_pawn(#[case] piece_type: PieceType) {
        let previous_state = factories::chessboard();

        // Move the black pawn that will be captured.
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Five, File::D);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a white pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Five, File::E);
        let piece = Piece::new(Colour::White, piece_type);
        current_state.add_piece(piece, &square).unwrap();

        let target_square = Square::new(Rank::Six, File::D);
        let en_passant = EnPassant::new(&piece, &square, &target_square);
        let chessboard_history = vec![previous_state, current_state];

        let result = en_passant.validate(&chessboard_history);

        let expected_error = EnPassantValidationError::OnlyAllowedForPawns;
        assert_eq!(result, Err(expected_error));
    }

    #[test]
    fn cannot_play_en_passant_if_pawn_double_advancement_was_not_previous_turn() {
        let previous_state = factories::chessboard();

        // Move the black pawn that will be captured.
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Five, File::D);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a white pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Five, File::E);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        current_state.add_piece(white_pawn, &square).unwrap();

        let target_square = Square::new(Rank::Six, File::D);
        let en_passant = EnPassant::new(&white_pawn, &square, &target_square);

        // Add an extra step in the history, so black's double advance wasn't previous turn.
        let chessboard_history = vec![previous_state, current_state.clone(), current_state];

        let result = en_passant.validate(&chessboard_history);

        let expected_error = EnPassantValidationError::OnlyAllowedAfterDoubleAdvancement;
        assert_eq!(result, Err(expected_error));
    }

    #[test]
    fn cannot_play_en_passant_from_invalid_starting_square() {
        let mut starting_position = HashMap::new();

        // Create an initial state with a black pawn at D5.
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let black_starting_square = Square::new(Rank::Seven, File::D);
        starting_position.insert(black_starting_square, black_pawn);

        // And a white pawn on its starting rank.
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let white_starting_square = Square::new(Rank::Three, File::E);
        starting_position.insert(white_starting_square, white_pawn);

        let previous_state = Chessboard::new(starting_position);
        let mut current_state = previous_state.clone();

        // Move the black pawn that will be captured to the same
        // rank as the white (an illegal move).
        let to_square = Square::new(Rank::Three, File::D);
        current_state
            .move_piece(&black_starting_square, &to_square)
            .unwrap();

        // Try and make an en passant with the pawn still on its starting rank.
        let target_square = Square::new(Rank::Four, File::D);
        let en_passant = EnPassant::new(&white_pawn, &white_starting_square, &target_square);
        let chessboard_history = vec![previous_state, current_state];

        let result = en_passant.validate(&chessboard_history);

        let expected_error = EnPassantValidationError::InvalidStartingSquare;
        assert_eq!(result, Err(expected_error));
    }
}
