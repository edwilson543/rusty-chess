use super::translation;
use crate::domain::gameplay::chess_set;

/// A move of a single piece from one square to another.
pub struct Move<'a> {
    pub piece: &'a chess_set::Piece,
    pub from_square: &'a chess_set::Square,
    pub to_square: &'a chess_set::Square,
    pub translation: translation::Translation,
}

impl<'a> Move<'a> {
    pub fn new(
        piece: &'a chess_set::Piece,
        from_square: &'a chess_set::Square,
        to_square: &'a chess_set::Square,
    ) -> Self {
        let translation =
            translation::Translation::from_move(from_square, to_square, piece.get_colour());

        Self {
            piece: piece,
            from_square: from_square,
            to_square: to_square,
            translation: translation,
        }
    }
}

/// Mechanism for defining whether a certain translation is allowed.
pub trait MoveRule {
    fn allows_move(&self, move_: &Move) -> bool;
}
