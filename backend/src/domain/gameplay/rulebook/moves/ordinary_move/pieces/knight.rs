use super::super::rule::OrdinaryMoveRule;
use super::super::OrdinaryMove;
use crate::domain::gameplay::rulebook::moves::translation;
use std::vec;

pub fn get_knight_move_rules() -> vec::IntoIter<Box<dyn OrdinaryMoveRule>> {
    let rules = vec![Box::new(LShapedJump) as Box<dyn OrdinaryMoveRule>];

    rules.into_iter()
}

struct LShapedJump;

impl OrdinaryMoveRule for LShapedJump {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool {
        let allowed_vectors = [
            translation::ChessVector::new(1, 2),
            translation::ChessVector::new(2, 1),
            translation::ChessVector::new(2, -1),
            translation::ChessVector::new(1, -2),
            translation::ChessVector::new(-1, -2),
            translation::ChessVector::new(-2, -1),
            translation::ChessVector::new(-2, 1),
            translation::ChessVector::new(-1, 2),
        ];

        let direction_allowed = allowed_vectors
            .into_iter()
            .any(|vector| chess_move.translation.vector == vector);

        // Knights can jump, so we don't check for obstruction.
        direction_allowed && chess_move.translation.scalar == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::OrdinaryMove;
    use crate::testing::factories;
    use rstest::rstest;

    fn is_move_allowed(chess_move: &OrdinaryMove) -> bool {
        let mut rules = get_knight_move_rules();
        rules.any(|rule| rule.allows_move(chess_move))
    }

    #[rstest]
    #[case::na3(Square::new(Rank::One, File::B), Square::new(Rank::Three, File::A))]
    #[case::undo_na3(Square::new(Rank::Three, File::A), Square::new(Rank::One, File::B))]
    #[case::nc3(Square::new(Rank::One, File::B), Square::new(Rank::Three, File::C))]
    #[case::undo_nc3(Square::new(Rank::Three, File::C), Square::new(Rank::One, File::B))]
    #[case::nf3(Square::new(Rank::One, File::G), Square::new(Rank::Three, File::F))]
    #[case::undo_nf3(Square::new(Rank::Three, File::F), Square::new(Rank::One, File::G))]
    #[case::nh3(Square::new(Rank::One, File::G), Square::new(Rank::Three, File::H))]
    #[case::undo_nh3(Square::new(Rank::Three, File::H), Square::new(Rank::One, File::G))]
    fn allows_white_knight_to_move_in_l_shape(
        #[case] from_square: Square,
        #[case] to_square: Square,
    ) {
        let knight = Piece::new(Colour::White, PieceType::Knight);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &knight, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[rstest]
    #[case::na6(Square::new(Rank::Eight, File::B), Square::new(Rank::Six, File::A))]
    #[case::undo_na6(Square::new(Rank::Six, File::A), Square::new(Rank::Eight, File::B))]
    #[case::nc6(Square::new(Rank::Eight, File::B), Square::new(Rank::Six, File::C))]
    #[case::undo_nc6(Square::new(Rank::Six, File::C), Square::new(Rank::Eight, File::B))]
    #[case::nf6(Square::new(Rank::Eight, File::G), Square::new(Rank::Six, File::F))]
    #[case::undo_nf6(Square::new(Rank::Six, File::F), Square::new(Rank::Eight, File::G))]
    #[case::nh6(Square::new(Rank::Eight, File::G), Square::new(Rank::Six, File::H))]
    #[case::undo_nh6(Square::new(Rank::Six, File::H), Square::new(Rank::Eight, File::G))]
    fn allows_black_knight_to_move_in_l_shape(
        #[case] from_square: Square,
        #[case] to_square: Square,
    ) {
        let knight = Piece::new(Colour::Black, PieceType::Knight);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &knight, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[rstest]
    #[case::forwards(Square::new(Rank::One, File::B), Square::new(Rank::Three, File::B))]
    #[case::sideways(Square::new(Rank::Three, File::C), Square::new(Rank::Three, File::A))]
    #[case::diagonal(Square::new(Rank::Five, File::E), Square::new(Rank::Seven, File::G))]
    fn disallowed_moves(#[case] from_square: Square, #[case] to_square: Square) {
        let knight = Piece::new(Colour::White, PieceType::Knight);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &knight, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }
}