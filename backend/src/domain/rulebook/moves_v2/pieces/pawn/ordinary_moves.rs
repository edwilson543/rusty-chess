use crate::domain::chess_set;
use crate::domain::rulebook::moves_v2::{chess_move, translation};
use std::vec;

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

pub struct AllowDoubleSquareForward;

impl chess_move::MoveRule for AllowDoubleSquareForward {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let chessboard = chessboard_history.last().unwrap();
        let forwards = translation::ChessVector::forwards(chess_move.piece.get_colour());

        let is_forwards = chess_move.translation.vector == forwards;
        let is_two_squares = chess_move.translation.scalar == 2;
        let is_first_move_for_pawn = is_first_move_for_pawn(&chess_move);
        let is_square_occupied = chessboard.is_square_occupied(&chess_move.to_square);

        let middle_square = forwards.apply_to_square(&chess_move.from_square);
        let is_obstructed = chessboard.is_square_occupied(&middle_square);

        is_forwards
            && is_two_squares
            && is_first_move_for_pawn
            && !is_square_occupied
            && !is_obstructed
    }
}

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

// Helpers.

fn is_first_move_for_pawn(chess_move: &chess_move::Move) -> bool {
    let starting_rank = match chess_move.piece.get_colour() {
        chess_set::Colour::White => &chess_set::Rank::Two,
        chess_set::Colour::Black => &chess_set::Rank::Seven,
    };
    chess_move.from_square.get_rank() == starting_rank
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
