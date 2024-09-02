use crate::domain::chess_set;
use crate::domain::chess_set::{Piece, Square};
use crate::domain::rulebook::moves_v2::chess_move::Move;
use crate::domain::rulebook::moves_v2::{chess_move, translation};
use std::collections::BTreeMap;

struct AllowEnPassant;

impl chess_move::MoveRule for AllowEnPassant {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        opponent_made_double_pawn_advancement_over_target_square(chess_move, chessboard_history)
            && is_translation_valid(&chess_move)
    }

    fn get_move_outcome(&self, chess_move: &Move) -> BTreeMap<Square, Option<Piece>> {
        todo!()
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

#[cfg(test)]
mod tests {
    use super::AllowEnPassant;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook::moves_v2::chess_move;
    use crate::domain::rulebook::moves_v2::chess_move::MoveRule;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::BTreeMap;

    #[rstest]
    #[case(File::C)]
    #[case(File::E)]
    fn white_can_play_en_passant(#[case] white_starting_file: File) {
        let previous_state = factories::chessboard();

        // Move the black pawn that will be captured.
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Five, File::D);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a white pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Five, File::C);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        current_state.add_piece(white_pawn, &square).unwrap();

        let target_square = Square::new(Rank::Six, File::D);
        let en_passant = chess_move::Move::new(white_pawn, square, target_square);
        let chessboard_history = vec![previous_state, current_state];

        let result = AllowEnPassant.allows_move(&en_passant, &chessboard_history);

        assert!(result);
    }

    #[rstest]
    #[case(File::F)]
    #[case(File::H)]
    fn black_can_play_en_passant(#[case] black_starting_file: File) {
        let previous_state = factories::chessboard();

        // Move the white pawn that will be captured.
        let from_square = Square::new(Rank::Two, File::G);
        let to_square = Square::new(Rank::Four, File::G);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a black pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Four, black_starting_file);
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        current_state.add_piece(black_pawn, &square).unwrap();

        let target_square = Square::new(Rank::Three, File::G);
        let en_passant = chess_move::Move::new(black_pawn, square, target_square);
        let chessboard_history = vec![previous_state, current_state];

        let is_allowed = AllowEnPassant.allows_move(&en_passant, &chessboard_history);

        assert!(is_allowed);
    }

    #[rstest]
    #[case::rook(PieceType::Rook)]
    #[case::bishop(PieceType::Bishop)]
    #[case::queen(PieceType::Queen)]
    fn cannot_play_en_passant_with_a_non_pawn(#[case] piece_type: PieceType) {
        let previous_state = factories::chessboard();

        // Move the black pawn that will be captured.
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Five, File::D);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a white pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Five, File::E);
        let piece = Piece::new(Colour::White, piece_type);
        current_state.add_piece(piece, &square).unwrap();

        let target_square = Square::new(Rank::Six, File::D);
        let en_passant = chess_move::Move::new(piece, square, target_square);
        let chessboard_history = vec![previous_state, current_state];

        let is_allowed = AllowEnPassant.allows_move(&en_passant, &chessboard_history);

        assert!(is_allowed);
    }

    #[test]
    fn cannot_play_en_passant_if_pawn_double_advancement_was_not_previous_turn() {
        let previous_state = factories::chessboard();

        // Move the black pawn that will be captured.
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Five, File::D);
        let mut current_state = previous_state.clone();
        current_state.move_piece(&from_square, &to_square).unwrap();

        // Artificially put a white pawn in a valid position to play an en passant.
        let square = Square::new(Rank::Five, File::E);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        current_state.add_piece(white_pawn, &square).unwrap();

        let target_square = Square::new(Rank::Six, File::D);
        let en_passant = chess_move::Move::new(white_pawn, square, target_square);

        // Add an extra step in the history, so black's double advance wasn't previous turn.
        let chessboard_history = vec![previous_state, current_state.clone(), current_state];

        let is_allowed = AllowEnPassant.allows_move(&en_passant, &chessboard_history);

        assert!(!is_allowed);
    }

    #[test]
    fn cannot_play_en_passant_from_invalid_starting_square() {
        let mut starting_position = BTreeMap::new();

        // Create an initial state with a black pawn at D5.
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let black_starting_square = Square::new(Rank::Seven, File::D);
        starting_position.insert(black_starting_square, black_pawn);

        // And a white pawn on its starting rank.
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let white_starting_square = Square::new(Rank::Three, File::E);
        starting_position.insert(white_starting_square, white_pawn);

        let previous_state = Chessboard::new(starting_position);
        let mut current_state = previous_state.clone();

        // Move the black pawn that will be captured to the same
        // rank as the white (an illegal move).
        let to_square = Square::new(Rank::Three, File::D);
        current_state
            .move_piece(&black_starting_square, &to_square)
            .unwrap();

        // Try and make an en passant with the pawn still on its starting rank.
        let target_square = Square::new(Rank::Four, File::D);
        let en_passant = chess_move::Move::new(white_pawn, white_starting_square, target_square);
        let chessboard_history = vec![previous_state, current_state];

        let is_allowed = AllowEnPassant.allows_move(&en_passant, &chessboard_history);

        assert!(!is_allowed);
    }
}
