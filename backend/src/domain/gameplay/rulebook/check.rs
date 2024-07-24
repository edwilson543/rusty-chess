use super::moves;
use super::moves::Move;
use crate::domain::gameplay::chess_set;

/// Test whether a move would leave a player in check.
///
/// The strategy is to:
/// * Provisionally apply the move to the chessboard
/// * Find the square the player's king is on
/// * Test whether any of the opponent's pieces can attack that square
pub fn would_player_be_left_in_check(
    player: &chess_set::Colour,
    chess_move: &Box<dyn Move>,
    chessboard: &chess_set::Chessboard,
) -> Result<bool, chess_set::ChessboardActionError> {
    let mut trial_chessboard = chessboard.clone();
    match chess_move.apply(&mut trial_chessboard) {
        Ok(_) => {}
        Err(error) => return Err(error),
    };
    Ok(is_player_in_check(&player, trial_chessboard))
}

pub fn is_player_in_check(player: &chess_set::Colour, chessboard: chess_set::Chessboard) -> bool {
    // Locate the king on the _trial_ chessboard, in case the king has moved.
    let king_location = chessboard.get_square_king_is_on(player);
    let opponent_player = player.swap();

    for (from_square, opponent_piece) in chessboard.get_pieces(opponent_player) {
        let potential_move =
            moves::OrdinaryMove::new(&chessboard, &opponent_piece, &from_square, &king_location);

        // Chessboard history isn't needed here, so we just supply an empty vector.
        let Err(error) = potential_move.validate(&vec![]) else {
            panic!("Potential move should be invalid, since it to opponent king's square.");
        };
        match error {
            moves::MoveValidationError::CannotCaptureOpponentKing => return true,
            _ => continue,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::would_player_be_left_in_check;
    use super::Move;
    use crate::domain::gameplay::chess_set::{
        Chessboard, ChessboardActionError, Colour, File, Piece, PieceType, Rank, Square,
    };
    use crate::domain::gameplay::rulebook;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::HashMap;

    // Check scenarios.

    #[test]
    fn king_cannot_move_into_check_from_rook() {
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

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::Black, &boxed_move, &chessboard);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn king_cannot_move_into_check_from_knight() {
        let mut position = HashMap::new();

        let black_knight = Piece::new(Colour::Black, PieceType::Knight);
        let knight_square = Square::new(Rank::Three, File::C);
        position.insert(knight_square, black_knight);

        let white_king = Piece::new(Colour::White, PieceType::King);
        let king_from_square = Square::new(Rank::Two, File::B);
        position.insert(king_from_square, white_king);
        let king_to_square = Square::new(Rank::One, File::B);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &white_king,
            &king_from_square,
            &king_to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result =
            would_player_be_left_in_check(&Colour::White, &Box::new(boxed_move), &chessboard);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn cannot_move_piece_shielding_king_from_check_from_bishop() {
        let mut position = HashMap::new();

        let white_bishop = Piece::new(Colour::White, PieceType::Bishop);
        let bishop_square = Square::new(Rank::Two, File::F);
        position.insert(bishop_square, white_bishop);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let king_square = Square::new(Rank::Five, File::C);
        position.insert(king_square, black_king);

        let shield_piece = Piece::new(Colour::Black, PieceType::Pawn);
        let shield_from_square = Square::new(Rank::Four, File::D);
        position.insert(shield_from_square, shield_piece);

        let shield_to_square = Square::new(Rank::Three, File::D);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &shield_piece,
            &shield_from_square,
            &shield_to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::Black, &boxed_move, &chessboard);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn cannot_move_piece_shielding_king_from_check_from_queen() {
        let mut position = HashMap::new();

        let black_queen = Piece::new(Colour::Black, PieceType::Bishop);
        let queen_square = Square::new(Rank::Eight, File::A);
        position.insert(queen_square, black_queen);

        let white_king = Piece::new(Colour::White, PieceType::King);
        let king_square = Square::new(Rank::One, File::H);
        position.insert(king_square, white_king);

        let shield_piece = Piece::new(Colour::White, PieceType::Pawn);
        let shield_from_square = Square::new(Rank::Two, File::G);
        position.insert(shield_from_square, shield_piece);

        let shield_to_square = Square::new(Rank::Three, File::G);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &shield_piece,
            &shield_from_square,
            &shield_to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::White, &boxed_move, &chessboard);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn king_cannot_move_into_check_from_pawn() {
        let mut position = HashMap::new();

        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let pawn_square = Square::new(Rank::Two, File::E);
        position.insert(pawn_square, black_pawn);

        let white_king = Piece::new(Colour::White, PieceType::King);
        let king_from_square = Square::new(Rank::One, File::E);
        position.insert(king_from_square, white_king);

        let king_to_square = Square::new(Rank::One, File::F);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &white_king,
            &king_from_square,
            &king_to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::White, &boxed_move, &chessboard);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn white_king_would_leave_black_king_in_check() {
        let mut position = HashMap::new();

        let white_king = Piece::new(Colour::White, PieceType::King);
        let white_king_square = Square::new(Rank::Four, File::D);
        position.insert(white_king_square, white_king);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_from_square = Square::new(Rank::Six, File::F);
        position.insert(black_king_from_square, black_king);

        let black_king_to_square = Square::new(Rank::Five, File::E);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &black_king,
            &black_king_from_square,
            &black_king_to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::Black, &boxed_move, &chessboard);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn king_must_move_out_of_check() {
        let mut position = HashMap::new();

        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let pawn_square = Square::new(Rank::Two, File::E);
        position.insert(pawn_square, black_pawn);

        // Place the white king in check.
        let white_king = Piece::new(Colour::White, PieceType::King);
        let king_square = Square::new(Rank::One, File::D);
        position.insert(king_square, white_king);

        // Try moving some other random piece, leaving the white king in check.
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let from_square = Square::new(Rank::Four, File::A);
        position.insert(from_square, white_pawn);
        let to_square = Square::new(Rank::Five, File::A);

        let chessboard = Chessboard::new(position);

        let chess_move =
            rulebook::OrdinaryMove::new(&chessboard, &white_pawn, &from_square, &to_square);

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::White, &boxed_move, &chessboard);

        assert_eq!(result, Ok(true));
    }

    // Non-check scenarios.
    #[test]
    fn piece_does_not_check_king_when_cannot_attack_square() {
        let mut position = HashMap::new();

        let white_rook = Piece::new(Colour::White, PieceType::Rook);
        let rook_square = Square::new(Rank::Two, File::F);
        position.insert(rook_square, white_rook);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let king_from_square = Square::new(Rank::Eight, File::D);
        position.insert(king_from_square, black_king);
        let king_to_square = Square::new(Rank::Eight, File::E);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &black_king,
            &king_from_square,
            &king_to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::Black, &boxed_move, &chessboard);

        assert_eq!(result, Ok(false));
    }

    #[rstest]
    #[case::white(Colour::White)]
    #[case::black(Colour::Black)]
    fn rook_does_not_check_king_when_obstructed(#[case] shield_piece_colour: Colour) {
        let mut position = HashMap::new();

        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        let rook_square = Square::new(Rank::Two, File::E);
        position.insert(rook_square, black_rook);

        // Place the white king in check.
        let white_king = Piece::new(Colour::White, PieceType::King);
        let king_from_square = Square::new(Rank::Seven, File::E);
        position.insert(king_from_square, white_king);
        let king_to_square = Square::new(Rank::Eight, File::E);

        // ... but shielded by some other piece.
        let shield_piece = Piece::new(shield_piece_colour, PieceType::Pawn);
        let shield_from_square = Square::new(Rank::Five, File::E);
        position.insert(shield_from_square, shield_piece);

        let chessboard = Chessboard::new(position);

        let chess_move = rulebook::OrdinaryMove::new(
            &chessboard,
            &white_king,
            &king_from_square,
            &king_to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::White, &boxed_move, &chessboard);

        assert_eq!(result, Ok(false));
    }

    // Invalid board state scenarios.
    #[test]
    fn error_if_chessboard_action_not_valid() {
        let position = HashMap::new();
        let empty_chessboard = Chessboard::new(position);

        let from_square = factories::some_square();
        let to_square = factories::some_other_square();
        let chess_move = rulebook::OrdinaryMove::new(
            &empty_chessboard,
            &factories::some_piece(),
            &from_square,
            &to_square,
        );

        let boxed_move = Box::new(chess_move) as Box<dyn Move>;
        let result = would_player_be_left_in_check(&Colour::White, &boxed_move, &empty_chessboard);

        let expected_error = Err(ChessboardActionError::SquareIsEmpty(from_square));
        assert_eq!(result, expected_error);
    }
}
