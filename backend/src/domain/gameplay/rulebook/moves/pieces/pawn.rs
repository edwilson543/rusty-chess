use super::super::{common, move_rule, translation};
use crate::domain::gameplay::chess_set;
use std::vec;

pub fn get_pawn_move_rules() -> vec::IntoIter<Box<dyn move_rule::MoveRule>> {
    let one_square_forwards_rule =
        common::SingleSquareMove::new(translation::ChessVector::new(0, 1));

    let rules = vec![
        // En passant is implemented elsewhere.
        Box::new(one_square_forwards_rule) as Box<dyn move_rule::MoveRule>,
        Box::new(TwoSquaresForwardMove) as Box<dyn move_rule::MoveRule>,
        // TODO -> pawn capture!
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

fn is_first_move_for_pawn(move_: &move_rule::Move) -> bool {
    match move_.piece.get_colour() {
        chess_set::Colour::White => move_.from_square.get_rank() == &chess_set::Rank::TWO,
        chess_set::Colour::Black => move_.from_square.get_rank() == &chess_set::Rank::SEVEN,
    }
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
        let from_square = Square::new(Rank::TWO, File::F);
        let to_square = Square::new(Rank::THREE, File::F);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn black_can_move_single_square_forwards() {
        let from_square = Square::new(Rank::SEVEN, File::C);
        let to_square = Square::new(Rank::SIX, File::C);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn white_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::TWO, File::A);
        let to_square = Square::new(Rank::FOUR, File::A);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn black_can_move_two_squares_forwards_from_starting_square() {
        let from_square = Square::new(Rank::SEVEN, File::E);
        let to_square = Square::new(Rank::FIVE, File::E);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(is_move_allowed(&move_));
    }

    #[test]
    fn can_capture_diagonally() {
        // TODO -> test both directions.
    }

    // Disallowed
    #[test]
    fn cannot_move_SIDEWAYS() {
        let from_square = Square::new(Rank::THREE, File::F);
        let to_square = Square::new(Rank::THREE, File::E);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }

    #[test]
    fn cannot_move_diagonally_without_capture() {
        let from_square = Square::new(Rank::TWO, File::F);
        let to_square = Square::new(Rank::THREE, File::E);
        let pawn = Piece::new(Colour::White, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }

    #[test]
    fn cannot_move_two_squares_forwards_when_has_already_moved() {
        let from_square = Square::new(Rank::SIX, File::C);
        let to_square = Square::new(Rank::FOUR, File::C);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }

    #[test]
    fn cannot_move_multiple_squares_forward() {
        let from_square = Square::new(Rank::TWO, File::B);
        let to_square = Square::new(Rank::FIVE, File::B);
        let pawn = Piece::new(Colour::Black, PieceType::Pawn);

        let chessboard = factories::chessboard();
        let move_ = Move::new(&chessboard, &pawn, &from_square, &to_square);

        assert!(!is_move_allowed(&move_));
    }
}
