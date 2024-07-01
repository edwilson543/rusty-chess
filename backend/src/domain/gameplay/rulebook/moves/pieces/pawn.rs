use super::super::{common, move_rule, translation};
use crate::domain::gameplay::chess_set;
use std::vec;

pub fn get_pawn_move_rules() -> vec::IntoIter<Box<dyn move_rule::MoveRule>> {
    let one_square_forwards_move =
        common::SingleSquareMove::new(translation::ChessVector::new(0, 1));

    // Note: En passant is implemented elsewhere.
    let rules = vec![
        Box::new(one_square_forwards_move) as Box<dyn move_rule::MoveRule>,
        Box::new(TwoSquaresForwardMove) as Box<dyn move_rule::MoveRule>,
        Box::new(ForwardsDiagonalCapture) as Box<dyn move_rule::MoveRule>,
    ];

    rules.into_iter()
}

struct TwoSquaresForwardMove;

impl move_rule::MoveRule for TwoSquaresForwardMove {
    fn allows_move(&self, move_: &move_rule::Move) -> bool {
        let forwards = translation::ChessVector::new(0, 1);

        let is_forwards = move_.translation.vector == forwards;
        let is_two_squares = move_.translation.scalar == 2;
        let is_first_move_for_pawn = is_first_move_for_pawn(move_);

        is_forwards && is_two_squares && is_first_move_for_pawn
    }
}

struct ForwardsDiagonalCapture;

impl move_rule::MoveRule for ForwardsDiagonalCapture {
    fn allows_move(&self, move_: &move_rule::Move) -> bool {
        let forwards_and_right = translation::ChessVector::new(1, 1);
        let forwards_and_left = translation::ChessVector::new(-1, 1);

        let is_forwards_diagonal = move_.translation.vector == forwards_and_right
            || move_.translation.vector == forwards_and_left;

        let is_capture = is_square_occupied_by_opponent_piece(move_);

        is_forwards_diagonal && move_.translation.scalar == 1 && is_capture
    }
}

fn is_first_move_for_pawn(move_: &move_rule::Move) -> bool {
    let starting_rank = match move_.piece.get_colour() {
        chess_set::Colour::White => &chess_set::Rank::Two,
        chess_set::Colour::Black => &chess_set::Rank::Seven,
    };
    move_.from_square.get_rank() == starting_rank
}

fn is_square_occupied_by_opponent_piece(move_: &move_rule::Move) -> bool {
    let Some(piece) = move_.chessboard.get_piece(&move_.to_square) else {
        return false;
    };
    piece.get_colour() != move_.piece.get_colour()
}

#[cfg(test)]
mod tests {
    use super::get_pawn_move_rules;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::move_rule::Move;
    use crate::testing::factories;

    fn is_move_allowed(move_: &Move) -> bool {
        let mut rules = get_pawn_move_rules();
        rules.any(|rule| rule.allows_move(move_))
    }

    // Allowed.
    #[test]
    fn white_can_move_single_square_forwards() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::F);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn black_can_move_single_square_forwards() {
        let from_square = Square::new(Rank::Seven, File::C);
        let to_square = Square::new(Rank::Six, File::C);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn white_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::Two, File::A);
        let to_square = Square::new(Rank::Four, File::A);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn black_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::Seven, File::E);
        let to_square = Square::new(Rank::Five, File::E);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn white_can_capture_diagonally() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::E);
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);

        let mut chessboard = factories::chessboard();
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);
        chessboard.add_piece(black_pawn, &to_square);

        let move_ = Move::new(&chessboard, &white_pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn black_can_capture_diagonally() {
        let from_square = Square::new(Rank::Seven, File::D);
        let to_square = Square::new(Rank::Six, File::E);
        let black_pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let mut chessboard = factories::chessboard();
        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        chessboard.add_piece(white_pawn, &to_square);

        let move_ = Move::new(&chessboard, &black_pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    // Disallowed
    #[test]
    fn cannot_move_sideways() {
        let from_square = Square::new(Rank::Three, File::F);
        let to_square = Square::new(Rank::Three, File::E);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }

    #[test]
    fn cannot_move_diagonally_without_capture() {
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::E);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }

    #[test]
    fn cannot_move_two_squares_forwards_when_has_already_moved() {
        let from_square = Square::new(Rank::Six, File::C);
        let to_square = Square::new(Rank::Four, File::C);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }

    #[test]
    fn cannot_move_multiple_squares_forward() {
        let from_square = Square::new(Rank::Two, File::B);
        let to_square = Square::new(Rank::Five, File::B);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }
}
