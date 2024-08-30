use super::check;
use crate::domain::chess_set;

use super::helpers;

pub fn is_player_checkmated(
    player: chess_set::Colour,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> bool {
    let chessboard = chessboard_history.last().unwrap();

    if !check::is_player_in_check(&player, chessboard.clone()) {
        return false;
    }

    let legal_moves = helpers::get_legal_moves(player, chessboard_history);
    legal_moves.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use std::collections::BTreeMap;

    // Checkmate scenarios.

    #[test]
    fn test_back_rank_mate() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::E);
        starting_position.insert(black_king_square, black_king);

        let white_rook = Piece::new(Colour::White, PieceType::Rook);
        let white_rook_square = Square::new(Rank::Eight, File::B);
        starting_position.insert(white_rook_square, white_rook);

        let white_queen = Piece::new(Colour::White, PieceType::Rook);
        let white_queen_square = Square::new(Rank::Seven, File::C);
        starting_position.insert(white_queen_square, white_queen);

        let chessboard = Chessboard::new(starting_position);

        assert!(is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    #[test]
    fn test_smothered_mate() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::H);
        starting_position.insert(black_king_square, black_king);

        // Smother the black king.
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        let black_rook_square = Square::new(Rank::Eight, File::G);
        starting_position.insert(black_rook_square, black_rook);
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let black_pawn_square = Square::new(Rank::Seven, File::G);
        starting_position.insert(black_pawn_square, black_pawn);
        let other_black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let other_black_pawn_square = Square::new(Rank::Seven, File::H);
        starting_position.insert(other_black_pawn_square, other_black_pawn);

        let white_knight = Piece::new(Colour::White, PieceType::Knight);
        let white_knight_square = Square::new(Rank::Seven, File::F);
        starting_position.insert(white_knight_square, white_knight);

        let chessboard = Chessboard::new(starting_position);

        assert!(is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    #[test]
    fn test_damianos_mate() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        let white_queen = Piece::new(Colour::White, PieceType::Queen);
        let white_queen_square = Square::new(Rank::Seven, File::A);
        starting_position.insert(white_queen_square, white_queen);

        // Protect the queen with a pawn.
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let white_pawn_square = Square::new(Rank::Six, File::B);
        starting_position.insert(white_pawn_square, white_pawn);

        let chessboard = Chessboard::new(starting_position);

        assert!(is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    #[test]
    fn test_blackburnes_mate() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::H);
        starting_position.insert(black_king_square, black_king);

        let white_bishop = Piece::new(Colour::White, PieceType::Bishop);
        let white_bishop_square = Square::new(Rank::Seven, File::F);
        starting_position.insert(white_bishop_square, white_bishop);

        let other_white_bishop = Piece::new(Colour::White, PieceType::Bishop);
        let other_white_bishop_square = Square::new(Rank::Five, File::E);
        starting_position.insert(other_white_bishop_square, other_white_bishop);

        let white_knight = Piece::new(Colour::White, PieceType::Knight);
        let white_knight_square = Square::new(Rank::Five, File::G);
        starting_position.insert(white_knight_square, white_knight);

        let chessboard = Chessboard::new(starting_position);

        assert!(is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    // Not checkmate scenarios.

    #[test]
    fn test_not_checkmate_when_not_in_check() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        let chessboard = Chessboard::new(starting_position);

        assert!(!is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    #[test]
    fn test_not_checkmate_when_king_can_move_out_of_check() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        let white_queen = Piece::new(Colour::White, PieceType::Queen);
        let white_queen_square = Square::new(Rank::One, File::A);
        starting_position.insert(white_queen_square, white_queen);

        let chessboard = Chessboard::new(starting_position);

        assert!(!is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    #[test]
    fn test_not_checkmate_when_king_can_capture_attacking_piece() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        let white_queen = Piece::new(Colour::White, PieceType::Queen);
        let white_queen_square = Square::new(Rank::Seven, File::B);
        starting_position.insert(white_queen_square, white_queen);

        let chessboard = Chessboard::new(starting_position);

        assert!(!is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    #[test]
    fn test_not_checkmate_when_another_piece_can_block_check() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        // The black rook can be moved to protect the king.
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        let black_rook_square = Square::new(Rank::Six, File::E);
        starting_position.insert(black_rook_square, black_rook);

        let white_rook = Piece::new(Colour::White, PieceType::Queen);
        let white_rook_square = Square::new(Rank::One, File::A);
        starting_position.insert(white_rook_square, white_rook);

        let other_white_rook = Piece::new(Colour::White, PieceType::Queen);
        let other_white_rook_square = Square::new(Rank::Three, File::B);
        starting_position.insert(other_white_rook_square, other_white_rook);

        let chessboard = Chessboard::new(starting_position);

        assert!(!is_player_checkmated(Colour::Black, &vec![chessboard]));
    }

    #[test]
    fn test_not_checkmate_when_another_piece_can_capture_attacking_piece() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        // The black queen can capture the attacking white rook.
        let black_queen = Piece::new(Colour::Black, PieceType::Queen);
        let black_queen_square = Square::new(Rank::Three, File::E);
        starting_position.insert(black_queen_square, black_queen);

        let white_rook = Piece::new(Colour::White, PieceType::Queen);
        let white_rook_square = Square::new(Rank::Three, File::A);
        starting_position.insert(white_rook_square, white_rook);

        let other_white_rook = Piece::new(Colour::White, PieceType::Queen);
        let other_white_rook_square = Square::new(Rank::Two, File::B);
        starting_position.insert(other_white_rook_square, other_white_rook);

        let chessboard = Chessboard::new(starting_position);

        assert!(!is_player_checkmated(Colour::Black, &vec![chessboard]));
    }
}
