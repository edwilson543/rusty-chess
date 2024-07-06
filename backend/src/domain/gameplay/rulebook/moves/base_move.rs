use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook::moves;

#[derive(Debug, PartialEq)]
pub enum CheckError {
    ChessboardActionError(chess_set::ChessboardActionError),
}

pub trait ChessMove {
    type Error;

    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError>;

    fn validate(&self, chessboard_history: &Vec<chess_set::Chessboard>) -> Result<(), Self::Error>;

    /// Test whether a move would leave a player in check.
    ///
    /// The strategy is to:
    /// * Provisionally apply the move to the chessboard
    /// * Find the square the player's king is on
    /// * Test whether any of the opponent's pieces can attack that square.
    fn would_player_be_left_in_check(
        &self,
        player: chess_set::Colour,
        chessboard: &chess_set::Chessboard,
    ) -> Result<bool, CheckError> {
        let mut trial_chessboard = chessboard.clone();
        match self.apply(&mut trial_chessboard) {
            Ok(_) => {}
            Err(error) => return Err(CheckError::ChessboardActionError(error)),
        }

        // Locate the king on the _trial_ chessboard, in case the king has moved.
        let king_location = trial_chessboard.get_square_king_is_on(player);
        let opponent_player = player.swap();

        for (from_square, opponent_piece) in chessboard.get_pieces(opponent_player) {
            let potential_move = moves::OrdinaryMove::new(
                &trial_chessboard,
                &opponent_piece,
                &from_square,
                &king_location,
            );

            // Chessboard history isn't needed here, so we just supply an empty vector.
            let Err(error) = potential_move.validate(&vec![]) else {
                panic!("Potential move should be invalid, since it to opponent king's square.");
            };
            match error {
                moves::MoveValidationError::CannotCaptureOpponentKing => return Ok(true),
                _ => continue,
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::ChessMove;
    use crate::domain::gameplay::chess_set::{
        Chessboard, Colour, File, Piece, PieceType, Rank, Square,
    };
    use crate::domain::gameplay::rulebook;
    use std::collections::HashMap;

    // Check scenarios.

    #[test]
    fn rook_can_check_king() {
        let mut position = HashMap::new();

        let white_rook = Piece::new(Colour::White, PieceType::Rook);
        let rook_square = Square::new(Rank::Two, File::F);
        position.insert(rook_square, white_rook);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let king_from_square = Square::new(Rank::Eight, File::E);
        position.insert(king_from_square, black_king);
        let king_to_square = Square::new(Rank::Eight, File::F);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &black_king,
            &king_from_square,
            &king_to_square,
        );

        let result = chess_move.would_player_be_left_in_check(Colour::Black, &chessboard);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn knight_can_check_king() {
        // TODO -> make sure it isn't always the king moving the player into check.
    }

    #[test]
    fn bishop_can_check_king() {}

    #[test]
    fn queen_can_check_king() {}

    #[test]
    fn pawn_can_check_king() {}

    #[test]
    fn white_king_would_leave_black_king_in_check() {}

    // Non-check scenarios.
    #[test]
    fn piece_cannot_check_king_of_own_colour() {}

    #[test]
    fn rook_cannot_check_king_when_obstructed() {}

    // Invalid board state scenarios.
    #[test]
    fn error_if_chessboard_action_not_valid() {}
}
