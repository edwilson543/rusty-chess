use crate::domain::chess_set;
use crate::domain::rulebook::moves::{chess_move, translation};

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

fn is_first_move_for_pawn(chess_move: &chess_move::Move) -> bool {
    let starting_rank = match chess_move.piece.get_colour() {
        chess_set::Colour::White => &chess_set::Rank::Two,
        chess_set::Colour::Black => &chess_set::Rank::Seven,
    };
    chess_move.from_square.get_rank() == starting_rank
}
