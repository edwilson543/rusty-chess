use super::super::rule::OrdinaryMoveRule;
use super::super::OrdinaryMove;
use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook::moves::translation::ChessVector;
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

        is_forwards && is_one_square
    }
}

struct TwoSquaresForwardMove;

impl OrdinaryMoveRule for TwoSquaresForwardMove {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool {
        let forwards = ChessVector::forwards(chess_move.piece.get_colour());

        let is_forwards = chess_move.translation.vector == forwards;
        let is_two_squares = chess_move.translation.scalar == 2;
        let is_first_move_for_pawn = is_first_move_for_pawn(chess_move);

        is_forwards && is_two_squares && is_first_move_for_pawn
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
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::OrdinaryMove;
    use crate::testing::factories;

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

    // Disallowed
    #[test]
    fn cannot_move_sideways() {
        let from_square = Square::new(Rank::Three, File::F);
        let to_square = Square::new(Rank::Three, File::E);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }

    #[test]
    fn cannot_move_diagonally_without_capture() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::E);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }

    #[test]
    fn cannot_move_two_squares_forwards_when_has_already_moved() {
        let from_square = Square::new(Rank::Six, File::C);
        let to_square = Square::new(Rank::Four, File::C);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }

    #[test]
    fn cannot_move_multiple_squares_forward() {
        let from_square = Square::new(Rank::Two, File::B);
        let to_square = Square::new(Rank::Five, File::B);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }
}
