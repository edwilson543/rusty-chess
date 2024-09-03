use super::diagonal_capture;
use super::double_square_forward;
use super::en_passant;
use super::single_square_forward;
use crate::domain::rulebook_v2::moves::chess_move;
use std::vec;

pub fn get_pawn_move_rules() -> vec::IntoIter<Box<dyn chess_move::MoveRule>> {
    vec![
        Box::new(single_square_forward::AllowSingleSquareForward) as Box<dyn chess_move::MoveRule>,
        Box::new(double_square_forward::AllowDoubleSquareForward) as Box<dyn chess_move::MoveRule>,
        Box::new(diagonal_capture::AllowDiagonalCapture) as Box<dyn chess_move::MoveRule>,
        Box::new(en_passant::AllowEnPassant) as Box<dyn chess_move::MoveRule>,
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::get_pawn_move_rules;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook_v2::moves::chess_move;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::BTreeMap;

    fn is_move_allowed(chess_move: &chess_move::Move, chessboard: &Chessboard) -> bool {
        let mut rules = get_pawn_move_rules();
        rules.any(|rule| rule.allows_move(&chess_move, &vec![chessboard.clone()]))
    }

    // Allowed.
    #[test]
    fn white_can_move_single_square_forwards() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::F);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(pawn, from_square, to_square);

        assert!(is_move_allowed(&chess_move, &chessboard));
    }

    #[test]
    fn black_can_move_single_square_forwards() {
        let from_square = Square::new(Rank::Seven, File::C);
        let to_square = Square::new(Rank::Six, File::C);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(pawn, from_square, to_square);

        assert!(is_move_allowed(&chess_move, &chessboard));
    }

    #[test]
    fn white_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::Two, File::A);
        let to_square = Square::new(Rank::Four, File::A);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(pawn, from_square, to_square);

        assert!(is_move_allowed(&chess_move, &chessboard));
    }

    #[test]
    fn black_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::Seven, File::E);
        let to_square = Square::new(Rank::Five, File::E);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(pawn, from_square, to_square);

        assert!(is_move_allowed(&chess_move, &chessboard));
    }

    #[test]
    fn white_can_capture_diagonally() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::E);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);

        let mut chessboard = factories::chessboard();
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let _ = chessboard.add_piece(black_pawn, &to_square);

        let chess_move = chess_move::Move::new(white_pawn, from_square, to_square);

        assert!(is_move_allowed(&chess_move, &chessboard));
    }

    #[test]
    fn black_can_capture_diagonally() {
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Six, File::E);
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let mut chessboard = factories::chessboard();
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let _ = chessboard.add_piece(white_pawn, &to_square);

        let chess_move = chess_move::Move::new(black_pawn, from_square, to_square);

        assert!(is_move_allowed(&chess_move, &chessboard));
    }

    #[rstest]
    #[case::sideways(
        Square::new(Rank::Three, File::F),
        Square::new(Rank::Three, File::E),
        Colour::White
    )]
    #[case::backwards_white(
        Square::new(Rank::Three, File::E),
        Square::new(Rank::Two, File::E),
        Colour::White
    )]
    #[case::backwards_black(
        Square::new(Rank::Three, File::F),
        Square::new(Rank::Four, File::F),
        Colour::Black
    )]
    #[case::diagonal_without_capture(
        Square::new(Rank::Two, File::F),
        Square::new(Rank::Three, File::E),
        Colour::White
    )]
    #[case::two_squares_forward_after_first_move(
        Square::new(Rank::Six, File::C),
        Square::new(Rank::Four, File::C),
        Colour::White
    )]
    #[case::three_squares_forward(
        Square::new(Rank::Two, File::B),
        Square::new(Rank::Five, File::B),
        Colour::White
    )]
    fn disallows_invalid_moves(
        #[case] from_square: Square,
        #[case] to_square: Square,
        #[case] colour: Colour,
    ) {
        let pawn = Piece::new(colour, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(pawn, from_square, to_square);

        assert!(!is_move_allowed(&chess_move, &chessboard));
    }

    #[rstest]
    #[case::white(Colour::White)]
    #[case::white(Colour::Black)]
    fn disallows_one_square_forward_capture(#[case] other_piece_colour: Colour) {
        let mut starting_position = BTreeMap::new();

        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let from_square = Square::new(Rank::Two, File::B);
        starting_position.insert(from_square, white_pawn);
        let to_square = Square::new(Rank::Three, File::B);

        let other_piece = Piece::new(other_piece_colour, PieceType::Bishop);
        starting_position.insert(to_square, other_piece);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = chess_move::Move::new(white_pawn, from_square, to_square);

        assert!(!is_move_allowed(&chess_move, &chessboard));
    }

    #[rstest]
    #[case::white(Colour::White)]
    #[case::white(Colour::Black)]
    fn disallows_double_square_forward_capture(#[case] other_piece_colour: Colour) {
        let mut starting_position = BTreeMap::new();

        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let from_square = Square::new(Rank::Two, File::B);
        starting_position.insert(from_square, white_pawn);
        let to_square = Square::new(Rank::Four, File::B);

        let other_piece = Piece::new(other_piece_colour, PieceType::Knight);
        starting_position.insert(to_square, other_piece);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = chess_move::Move::new(white_pawn, from_square, to_square);

        assert!(!is_move_allowed(&chess_move, &chessboard));
    }

    #[rstest]
    #[case::white(Colour::White)]
    #[case::white(Colour::Black)]
    fn disallows_double_square_forward_advancement_over_other_piece(
        #[case] blocking_piece_colour: Colour,
    ) {
        let mut starting_position = BTreeMap::new();

        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let from_square = Square::new(Rank::Two, File::B);
        starting_position.insert(from_square, white_pawn);
        let to_square = Square::new(Rank::Four, File::B);

        let middle_square = Square::new(Rank::Three, File::B);
        let blocking_piece = Piece::new(blocking_piece_colour, PieceType::Pawn);
        starting_position.insert(middle_square, blocking_piece);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = chess_move::Move::new(white_pawn, from_square, to_square);

        assert!(!is_move_allowed(&chess_move, &chessboard));
    }
}

#[cfg(test)]
mod en_passant_tests {
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook_v2::moves::chess_move;
    use crate::domain::rulebook_v2::moves::chess_move::MoveRule;
    use crate::domain::rulebook_v2::moves::pieces::pawn::get_pawn_move_rules;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::BTreeMap;

    fn is_move_allowed(
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<Chessboard>,
    ) -> bool {
        let mut rules = get_pawn_move_rules();
        rules.any(|rule| rule.allows_move(&chess_move, chessboard_history))
    }

    fn get_allowing_rule(
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<Chessboard>,
    ) -> Option<Box<dyn chess_move::MoveRule>> {
        let rules = get_pawn_move_rules();

        for rule in get_pawn_move_rules() {
            if rule.allows_move(chess_move, chessboard_history) {
                return Some(rule);
            }
        }
        None
    }

    #[rstest]
    #[case(File::C)]
    #[case(File::E)]
    fn white_can_play_en_passant(#[case] white_starting_file: File) {
        let previous_state = factories::chessboard();

        // Move the black pawn that will be captured.
        let captured_pawn_from_square = Square::new(Rank::Seven, File::D);
        let captured_pawn_to_square = Square::new(Rank::Five, File::D);
        let mut current_state = previous_state.clone();
        current_state
            .move_piece(&captured_pawn_from_square, &captured_pawn_to_square)
            .unwrap();

        // Artificially put a white pawn in a valid position to play an en passant.
        let white_pawn_from_square = Square::new(Rank::Five, File::C);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        current_state
            .add_piece(white_pawn, &white_pawn_from_square)
            .unwrap();

        let white_pawn_to_square = Square::new(Rank::Six, File::D);
        let en_passant =
            chess_move::Move::new(white_pawn, white_pawn_from_square, white_pawn_to_square);
        let chessboard_history = vec![previous_state, current_state];

        assert!(is_move_allowed(&en_passant, &chessboard_history));

        let allowing_move = get_allowing_rule(&en_passant, &chessboard_history);

        let move_outcome = allowing_move.unwrap().get_move_outcome(&en_passant);
        assert_eq!(move_outcome.get(&white_pawn_from_square).unwrap(), &None);
        assert_eq!(
            move_outcome.get(&white_pawn_to_square).unwrap(),
            &Some(white_pawn)
        );
        assert_eq!(move_outcome.get(&captured_pawn_to_square).unwrap(), &None);
    }

    #[rstest]
    #[case(File::F)]
    #[case(File::H)]
    fn black_can_play_en_passant(#[case] black_starting_file: File) {
        let previous_state = factories::chessboard();

        // Move the white pawn that will be captured.
        let white_pawn_from_square = Square::new(Rank::Two, File::G);
        let white_pawn_to_square = Square::new(Rank::Four, File::G);
        let mut current_state = previous_state.clone();
        current_state
            .move_piece(&white_pawn_from_square, &white_pawn_to_square)
            .unwrap();

        // Artificially put a black pawn in a valid position to play an en passant.
        let black_pawn_from_square = Square::new(Rank::Four, black_starting_file);
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        current_state
            .add_piece(black_pawn, &black_pawn_from_square)
            .unwrap();

        let black_pawn_to_square = Square::new(Rank::Three, File::G);
        let en_passant =
            chess_move::Move::new(black_pawn, black_pawn_from_square, black_pawn_to_square);
        let chessboard_history = vec![previous_state, current_state];

        assert!(is_move_allowed(&en_passant, &chessboard_history));

        let allowing_move = get_allowing_rule(&en_passant, &chessboard_history);

        let move_outcome = allowing_move.unwrap().get_move_outcome(&en_passant);
        assert_eq!(move_outcome.get(&black_pawn_from_square).unwrap(), &None);
        assert_eq!(
            move_outcome.get(&black_pawn_to_square).unwrap(),
            &Some(black_pawn)
        );
        assert_eq!(move_outcome.get(&white_pawn_to_square).unwrap(), &None);
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

        assert!(is_move_allowed(&en_passant, &chessboard_history));
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

        assert!(!is_move_allowed(&en_passant, &chessboard_history));
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

        assert!(!is_move_allowed(&en_passant, &chessboard_history));
    }
}
