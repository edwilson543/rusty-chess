use super::super::{common, move_rule, translation};
use std::vec;

pub fn get_rook_move_rules() -> vec::IntoIter<Box<dyn move_rule::MoveRule>> {
    let vectors = [
        translation::ChessVector::new(0, 1),
        translation::ChessVector::new(1, 0),
        translation::ChessVector::new(0, -1),
        translation::ChessVector::new(-1, 0),
    ];

    let mut rules: Vec<Box<dyn move_rule::MoveRule>> = vec![];
    for vector in vectors {
        let rule = common::MultiSquareMove::new(vector);
        rules.push(Box::new(rule) as Box<dyn move_rule::MoveRule>);
    }

    rules.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::move_rule::Move;
    use crate::testing::factories;
    use rstest::rstest;

    fn is_move_allowed(chess_move: &Move) -> bool {
        let mut rules = get_rook_move_rules();
        rules.any(|rule| rule.allows_move(chess_move))
    }

    #[rstest]
    #[case::forwards(Square::new(Rank::One, File::C), Square::new(Rank::Three, File::C))]
    #[case::right(Square::new(Rank::Five, File::A), Square::new(Rank::Five, File::H))]
    #[case::backwards(Square::new(Rank::Seven, File::F), Square::new(Rank::Two, File::F))]
    #[case::left(Square::new(Rank::Three, File::G), Square::new(Rank::Three, File::F))]
    fn allows_rook_to_move_within_plus(#[case] from_square: Square, #[case] to_square: Square) {
        let rook = Piece::new(Colour::White, PieceType::Rook);

        let chessboard = factories::chessboard();
        let chess_move = Move::new(&chessboard, &rook, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[rstest]
    #[case::diagonal(Square::new(Rank::One, File::B), Square::new(Rank::Two, File::C))]
    #[case::l_shaped(Square::new(Rank::Five, File::E), Square::new(Rank::Seven, File::F))]
    fn disallowed_moves(#[case] from_square: Square, #[case] to_square: Square) {
        let rook = Piece::new(Colour::White, PieceType::Rook);

        let chessboard = factories::chessboard();
        let chess_move = Move::new(&chessboard, &rook, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }
}
