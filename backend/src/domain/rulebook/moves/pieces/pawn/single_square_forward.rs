use crate::domain::chess_set;
use crate::domain::rulebook::moves::{chess_move, translation};

pub struct AllowSingleSquareForward;

impl chess_move::MoveRule for AllowSingleSquareForward {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let chessboard = chessboard_history.last().unwrap();
        let forwards = translation::ChessVector::forwards(chess_move.piece.get_colour());

        let is_forwards = chess_move.translation.vector == forwards;
        let is_single_square = chess_move.translation.scalar == 1;
        let is_square_occupied = chessboard.is_square_occupied(&chess_move.to_square);

        is_forwards && is_single_square && !is_square_occupied
    }
}
