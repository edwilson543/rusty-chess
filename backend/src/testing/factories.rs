use crate::domain::gameplay::chess_set;

pub fn some_square() -> chess_set::Square {
    chess_set::Square::new(chess_set::Rank::ONE, chess_set::File::A)
}

pub fn some_other_square() -> chess_set::Square {
    chess_set::Square::new(chess_set::Rank::TWO, chess_set::File::B)
}

pub fn some_piece() -> chess_set::Piece {
    chess_set::Piece::new(chess_set::Colour::Black, chess_set::PieceType::King)
}

pub fn some_other_piece() -> chess_set::Piece {
    chess_set::Piece::new(chess_set::Colour::White, chess_set::PieceType::Rook)
}
