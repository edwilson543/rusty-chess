use crate::domain::chess_set;

pub fn is_insufficient_material_draw(chessboard: &chess_set::Chessboard) -> bool {
    let n_white_pieces = chessboard.get_pieces(chess_set::Colour::White).len();
    let n_black_pieces = chessboard.get_pieces(chess_set::Colour::Black).len();

    n_white_pieces == 1 && n_black_pieces == 1
}

#[cfg(test)]
mod tests {
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use std::collections::BTreeMap;

    use super::is_insufficient_material_draw;
    use crate::testing::factories;

    #[test]
    fn draw_when_one_player_has_insufficient_material() {
        let mut starting_position = BTreeMap::new();

        let white_king = Piece::new(Colour::White, PieceType::King);
        let white_king_square = Square::new(Rank::One, File::A);
        starting_position.insert(white_king_square, white_king);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        let chessboard = Chessboard::new(starting_position);

        assert!(is_insufficient_material_draw(&chessboard));
    }

    #[test]
    fn not_draw_when_both_players_have_sufficient_material() {
        let chessboard = factories::chessboard();

        assert!(!is_insufficient_material_draw(&chessboard));
    }
}
