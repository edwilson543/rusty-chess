use super::super::translation;
use super::rule;
use crate::domain::gameplay::rulebook::OrdinaryMove;
use std::vec;

pub fn get_king_move_rules() -> vec::IntoIter<Box<dyn rule::OrdinaryMoveRule>> {
    vec![Box::new(KingMoveRule) as Box<dyn rule::OrdinaryMoveRule>].into_iter()
}

struct KingMoveRule;

impl rule::OrdinaryMoveRule for KingMoveRule {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool {
        let vectors = [
            translation::ChessVector::new(0, 1),
            translation::ChessVector::new(1, 1),
            translation::ChessVector::new(1, 0),
            translation::ChessVector::new(1, -1),
            translation::ChessVector::new(0, -1),
            translation::ChessVector::new(-1, -1),
            translation::ChessVector::new(-1, 0),
            translation::ChessVector::new(-1, 1),
        ];

        let vector_valid = vectors
            .into_iter()
            .any(|vector| vector == chess_move.translation.vector);

        vector_valid && chess_move.translation.scalar == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::ordinary_move::OrdinaryMove;
    use crate::testing::factories;
    use rstest::rstest;

    fn is_move_allowed(chess_move: &OrdinaryMove) -> bool {
        let mut rules = get_king_move_rules();
        rules.any(|rule| rule.allows_move(chess_move))
    }

    #[rstest]
    #[case::forwards(Square::new(Rank::One, File::A), Square::new(Rank::Two, File::A))]
    #[case::forwards_and_right(Square::new(Rank::One, File::A), Square::new(Rank::Two, File::B))]
    #[case::right(Square::new(Rank::Eight, File::A), Square::new(Rank::Eight, File::B))]
    #[case::back_and_right(Square::new(Rank::Eight, File::A), Square::new(Rank::Seven, File::B))]
    #[case::back(Square::new(Rank::Seven, File::F), Square::new(Rank::Six, File::F))]
    #[case::back_and_left(Square::new(Rank::Seven, File::F), Square::new(Rank::Six, File::E))]
    #[case::left(Square::new(Rank::Two, File::H), Square::new(Rank::Two, File::G))]
    #[case::forwards_and_left(Square::new(Rank::Two, File::H), Square::new(Rank::Three, File::G))]
    fn allows_bishop_to_move_diagonally(#[case] from_square: Square, #[case] to_square: Square) {
        let king = Piece::new(Colour::White, PieceType::King);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &king, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[rstest]
    #[case::forwards_multiple(Square::new(Rank::One, File::B), Square::new(Rank::Three, File::B))]
    #[case::sideways_multiple(Square::new(Rank::Three, File::C), Square::new(Rank::Three, File::G))]
    #[case::l_shaped(Square::new(Rank::Five, File::E), Square::new(Rank::Seven, File::F))]
    fn disallowed_moves(#[case] from_square: Square, #[case] to_square: Square) {
        let king = Piece::new(Colour::White, PieceType::King);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &king, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }
}
