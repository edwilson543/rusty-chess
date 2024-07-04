use super::base_move;
use super::translation;
use crate::domain::gameplay::chess_set;

/// A move of a single piece from one square to another.
#[derive(Clone)]
pub struct OrdinaryMove {
    pub chessboard: chess_set::Chessboard,
    pub piece: chess_set::Piece,
    pub from_square: chess_set::Square,
    pub to_square: chess_set::Square,
    pub translation: translation::Translation,
}

impl OrdinaryMove {
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

impl base_move::ChessMove for OrdinaryMove {
    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError> {
        chessboard.move_piece(&self.from_square, &self.to_square)
    }
}
