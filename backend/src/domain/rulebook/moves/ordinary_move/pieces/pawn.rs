use crate::domain::chess_set;
use crate::domain::rulebook::moves::ordinary_move::ordinary_move::OrdinaryMove;
use crate::domain::rulebook::moves::ordinary_move::rule::OrdinaryMoveRule;
use crate::domain::rulebook::moves::translation::ChessVector;
use std::vec;

pub fn get_pawn_move_rules() -> vec::IntoIter<Box<dyn OrdinaryMoveRule>> {
    // Note: En passant is implemented elsewhere.
    let rules = vec![
        Box::new(OneSquaresForwardMove) as Box<dyn OrdinaryMoveRule>,
        Box::new(TwoSquaresForwardMove) as Box<dyn OrdinaryMoveRule>,
        Box::new(ForwardsDiagonalCapture) as Box<dyn OrdinaryMoveRule>,
    ];

    rules.into_iter()
}

struct OneSquaresForwardMove;

impl OrdinaryMoveRule for OneSquaresForwardMove {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool {
        let forwards = ChessVector::forwards(chess_move.piece.get_colour());

        let is_forwards = chess_move.translation.vector == forwards;
        let is_one_square = chess_move.translation.scalar == 1;
        let is_square_occupied = chess_move
            .chessboard
            .is_square_occupied(&chess_move.to_square);

        is_forwards && is_one_square && !is_square_occupied
    }
}

struct TwoSquaresForwardMove;

impl OrdinaryMoveRule for TwoSquaresForwardMove {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool {
        let forwards = ChessVector::forwards(chess_move.piece.get_colour());

        let is_forwards = chess_move.translation.vector == forwards;
        let is_two_squares = chess_move.translation.scalar == 2;
        let is_first_move_for_pawn = is_first_move_for_pawn(chess_move);
        let is_square_occupied = chess_move
            .chessboard
            .is_square_occupied(&chess_move.to_square);

        is_forwards && is_two_squares && is_first_move_for_pawn && !is_square_occupied
    }
}

struct ForwardsDiagonalCapture;

impl OrdinaryMoveRule for ForwardsDiagonalCapture {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool {
        let forwards = ChessVector::forwards(chess_move.piece.get_colour());
        let right = ChessVector::right(chess_move.piece.get_colour());

        let forwards_and_right = forwards + right;
        let forwards_and_left = forwards - right;

        let is_forwards_diagonal = chess_move.translation.vector == forwards_and_right
            || chess_move.translation.vector == forwards_and_left;

        let is_capture = is_square_occupied_by_opponent_piece(chess_move);

        is_forwards_diagonal && chess_move.translation.scalar == 1 && is_capture
    }
}

fn is_first_move_for_pawn(chess_move: &OrdinaryMove) -> bool {
    let starting_rank = match chess_move.piece.get_colour() {
        chess_set::Colour::White => &chess_set::Rank::Two,
        chess_set::Colour::Black => &chess_set::Rank::Seven,
    };
    chess_move.from_square.get_rank() == starting_rank
}

fn is_square_occupied_by_opponent_piece(chess_move: &OrdinaryMove) -> bool {
    let Some(piece) = chess_move.chessboard.get_piece(&chess_move.to_square) else {
        return false;
    };
    piece.get_colour() != chess_move.piece.get_colour()
}

#[cfg(test)]
mod tests {
    use super::get_pawn_move_rules;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook::moves::OrdinaryMove;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::BTreeMap;

    fn is_move_allowed(chess_move: &OrdinaryMove) -> bool {
        let mut rules = get_pawn_move_rules();
        rules.any(|rule| rule.allows_move(chess_move))
    }

    // Allowed.
    #[test]
    fn white_can_move_single_square_forwards() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::F);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[test]
    fn black_can_move_single_square_forwards() {
        let from_square = Square::new(Rank::Seven, File::C);
        let to_square = Square::new(Rank::Six, File::C);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[test]
    fn white_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::Two, File::A);
        let to_square = Square::new(Rank::Four, File::A);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[test]
    fn black_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::Seven, File::E);
        let to_square = Square::new(Rank::Five, File::E);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[test]
    fn white_can_capture_diagonally() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::E);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);

        let mut chessboard = factories::chessboard();
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        let _ = chessboard.add_piece(black_pawn, &to_square);

        let chess_move = OrdinaryMove::new(&chessboard, &white_pawn, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[test]
    fn black_can_capture_diagonally() {
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Six, File::E);
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let mut chessboard = factories::chessboard();
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let _ = chessboard.add_piece(white_pawn, &to_square);

        let chess_move = OrdinaryMove::new(&chessboard, &black_pawn, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
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
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
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
        let chess_move = OrdinaryMove::new(&chessboard, &white_pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
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
        let chess_move = OrdinaryMove::new(&chessboard, &white_pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }
}
