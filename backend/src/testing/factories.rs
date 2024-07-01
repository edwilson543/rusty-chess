use crate::domain::gameplay;
use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook;

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

pub fn chessboard() -> chess_set::Chessboard {
    let starting_position = rulebook::get_official_starting_position();
    chess_set::Chessboard::new(starting_position)
}

pub fn player() -> gameplay::Player {
    gameplay::Player::new()
}

pub fn game() -> gameplay::Game {
    let white_player = player();
    let black_player = player();
    gameplay::Game::new(white_player, black_player)
}
