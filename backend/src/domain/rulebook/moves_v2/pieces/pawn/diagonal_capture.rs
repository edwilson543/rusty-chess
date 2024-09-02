use crate::domain::chess_set;
use crate::domain::rulebook::moves_v2::{chess_move, translation};

pub struct AllowDiagonalCapture;

impl chess_move::MoveRule for AllowDiagonalCapture {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let chessboard = chessboard_history.last().unwrap();
        let forwards = translation::ChessVector::forwards(chess_move.piece.get_colour());
        let right = translation::ChessVector::right(chess_move.piece.get_colour());

        let forwards_and_right = forwards + right;
        let forwards_and_left = forwards - right;

        let is_forwards_diagonal = chess_move.translation.vector == forwards_and_right
            || chess_move.translation.vector == forwards_and_left;

        let is_capture = is_square_occupied_by_opponent_piece(&chess_move, chessboard);

        is_forwards_diagonal && chess_move.translation.scalar == 1 && is_capture
    }
}

fn is_square_occupied_by_opponent_piece(
    chess_move: &chess_move::Move,
    chessboard: &chess_set::Chessboard,
) -> bool {
    let Some(piece) = chessboard.get_piece(&chess_move.to_square) else {
        return false;
    };
    piece.get_colour() != chess_move.piece.get_colour()
}
