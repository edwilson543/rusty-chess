use crate::domain::chess_set;

pub trait ChessboardEvaluator {
    fn evaluate_position(
        &self,
        chessboard: chess_set::Chessboard,
        for_colour: &chess_set::Colour,
    ) -> i32;
}
