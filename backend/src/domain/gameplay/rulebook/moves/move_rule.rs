use super::translation;
use crate::domain::gameplay::chess_set;

/// A move of a single piece from one square to another.
pub struct Move {
    pub chessboard: chess_set::Chessboard,
    pub piece: chess_set::Piece,
    pub from_square: chess_set::Square,
    pub to_square: chess_set::Square,
    pub translation: translation::Translation,
}

impl Move {
    pub fn new(
        chessboard: &chess_set::Chessboard,
        piece: &chess_set::Piece,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Self {
        let translation =
            translation::Translation::from_move(from_square, to_square, piece.get_colour());

        Self {
            chessboard: chessboard.clone(),
            piece: piece.clone(),
            from_square: from_square.clone(),
            to_square: to_square.clone(),
            translation: translation,
        }
    }
}

/// Mechanism for defining whether a certain translation is allowed.
pub trait MoveRule {
    fn allows_move(&self, move_: &Move) -> bool;
}
