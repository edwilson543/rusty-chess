use crate::domain::{chess_set, rulebook_v2};

pub fn some_square() -> chess_set::Square {
    chess_set::Square::new(chess_set::Rank::One, chess_set::File::A)
}

pub fn some_other_square() -> chess_set::Square {
    chess_set::Square::new(chess_set::Rank::Two, chess_set::File::B)
}

pub fn some_piece() -> chess_set::Piece {
    chess_set::Piece::new(chess_set::Colour::Black, chess_set::PieceType::King)
}

pub fn some_other_piece() -> chess_set::Piece {
    chess_set::Piece::new(chess_set::Colour::White, chess_set::PieceType::Rook)
}

pub fn chessboard() -> chess_set::Chessboard {
    let starting_position = rulebook_v2::get_official_starting_position();
    chess_set::Chessboard::new(starting_position)
}
