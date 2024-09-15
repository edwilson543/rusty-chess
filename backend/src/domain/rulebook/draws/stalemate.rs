use crate::domain::chess_set;
use crate::domain::rulebook;

pub fn is_stalemate(
    to_play_colour: chess_set::Colour,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> bool {
    rulebook::get_legal_moves(to_play_colour, chessboard_history).len() == 0
}

#[cfg(test)]
mod tests {
    use super::is_stalemate;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::testing::factories;
    use std::collections::BTreeMap;

    #[test]
    fn is_stalemate_when_player_cannot_move() {
        let mut starting_position = BTreeMap::new();

        // Corner the white king using a rook and a queen.
        let white_king = Piece::new(Colour::White, PieceType::King);
        let white_king_square = Square::new(Rank::One, File::A);
        starting_position.insert(white_king_square, white_king);

        let black_queen = Piece::new(Colour::Black, PieceType::Queen);
        let black_queen_square = Square::new(Rank::Two, File::H);
        starting_position.insert(black_queen_square, black_queen);

        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        let black_rook_square = Square::new(Rank::Eight, File::B);
        starting_position.insert(black_rook_square, black_rook);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::H);
        starting_position.insert(black_king_square, black_king);

        let chessboard = Chessboard::new(starting_position);

        assert!(is_stalemate(Colour::White, &vec![chessboard.clone()]));
        assert!(!is_stalemate(Colour::Black, &vec![chessboard]))
    }

    #[test]
    fn is_not_stalemate_when_player_can_move() {
        let chessboard = factories::chessboard();

        assert!(!is_stalemate(Colour::White, &vec![chessboard.clone()]));
        assert!(!is_stalemate(Colour::Black, &vec![chessboard]))
    }
}
