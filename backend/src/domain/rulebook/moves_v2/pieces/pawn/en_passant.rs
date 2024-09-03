use crate::domain::chess_set;
use crate::domain::rulebook::moves_v2::{chess_move, translation};
use std::collections::BTreeMap;

pub struct AllowEnPassant;

impl chess_move::MoveRule for AllowEnPassant {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        opponent_made_double_pawn_advancement_over_target_square(chess_move, chessboard_history)
            && is_translation_valid(&chess_move)
    }

    fn get_move_outcome(
        &self,
        chess_move: &chess_move::Move,
    ) -> BTreeMap<chess_set::Square, Option<chess_set::Piece>> {
        let mut outcome = BTreeMap::new();

        outcome.insert(chess_move.from_square, None);
        outcome.insert(chess_move.to_square, Some(chess_move.piece));

        let captured_piece_square = get_square_captured_pawn_should_have_moved_to(chess_move);
        outcome.insert(captured_piece_square, None);

        outcome
    }
}

// En passant is only allowed immediately after the opponent makes a double pawn advancement.
fn opponent_made_double_pawn_advancement_over_target_square(
    chess_move: &chess_move::Move,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> bool {
    let history_length = chessboard_history.len();
    if history_length < 2 {
        return false;
    }

    let opponent_colour = &chess_move.piece.get_colour().swap();

    let previous_state = &chessboard_history[history_length - 2];
    let from_square = get_square_captured_pawn_should_have_moved_from(&chess_move);
    let opponent_pawn_moved_from_expected_square =
        is_piece_at_square_opponent_pawn(previous_state, opponent_colour, &from_square);

    let current_state = chessboard_history.last().unwrap();
    let to_square = get_square_captured_pawn_should_have_moved_to(chess_move);
    let opponent_pawn_moved_to_expected_square =
        is_piece_at_square_opponent_pawn(current_state, opponent_colour, &to_square);

    opponent_pawn_moved_from_expected_square && opponent_pawn_moved_to_expected_square
}

fn is_piece_at_square_opponent_pawn(
    chessboard: &chess_set::Chessboard,
    opponent_colour: &chess_set::Colour,
    square: &chess_set::Square,
) -> bool {
    let Some(piece) = chessboard.get_piece(square) else {
        return false;
    };
    let is_opponent = piece.get_colour() == opponent_colour;
    let is_pawn = piece.get_piece_type() == &chess_set::PieceType::Pawn;
    is_opponent && is_pawn
}

fn get_square_captured_pawn_should_have_moved_to(
    chess_move: &chess_move::Move,
) -> chess_set::Square {
    let rank = chess_move.from_square.get_rank();
    let file = chess_move.to_square.get_file();
    chess_set::Square::new(rank.clone(), file.clone())
}

fn get_square_captured_pawn_should_have_moved_from(
    chess_move: &chess_move::Move,
) -> chess_set::Square {
    let rank = match chess_move.piece.get_colour() {
        // Opponent pawn should have moved from their starting rank.
        &chess_set::Colour::White => chess_set::Rank::Seven,
        &chess_set::Colour::Black => chess_set::Rank::Two,
    };
    let file = chess_move.to_square.get_file();
    chess_set::Square::new(rank.clone(), file.clone())
}

fn is_translation_valid(chess_move: &chess_move::Move) -> bool {
    // En passant can only be made after a pawn has advanced exactly 3 squares.
    let starting_rank_valid = match chess_move.piece.get_colour() {
        chess_set::Colour::White => chess_move.from_square.get_rank() == &chess_set::Rank::Five,
        chess_set::Colour::Black => chess_move.from_square.get_rank() == &chess_set::Rank::Four,
    };

    // An en passant must move the pawn diagonally forward one square.
    let forwards = translation::ChessVector::forwards(chess_move.piece.get_colour());
    let right = translation::ChessVector::right(chess_move.piece.get_colour());
    let forwards_and_right = forwards + right;
    let forwards_and_left = forwards - right;

    let is_forwards_diagonal = chess_move.translation.vector == forwards_and_right
        || chess_move.translation.vector == forwards_and_left;

    starting_rank_valid && is_forwards_diagonal
}
