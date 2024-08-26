use crate::domain::rulebook::moves::ordinary_move::multi_square_move::MultiSquareMove;
use crate::domain::rulebook::moves::ordinary_move::rule::OrdinaryMoveRule;
use crate::domain::rulebook::moves::translation;
use std::vec;

pub fn get_bishop_move_rules() -> vec::IntoIter<Box<dyn OrdinaryMoveRule>> {
    let diagonals = [
        translation::ChessVector::new(1, 1),
        translation::ChessVector::new(1, -1),
        translation::ChessVector::new(-1, -1),
        translation::ChessVector::new(-1, 1),
    ];

    let mut rules: Vec<Box<dyn OrdinaryMoveRule>> = vec![];
    for diagonal in diagonals {
        let rule = MultiSquareMove::new(diagonal);
        rules.push(Box::new(rule) as Box<dyn OrdinaryMoveRule>);
    }

    rules.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook::moves::OrdinaryMove;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::BTreeMap;

    fn is_move_allowed(chess_move: &OrdinaryMove) -> bool {
        let mut rules = get_bishop_move_rules();
        rules.any(|rule| rule.allows_move(chess_move))
    }

    #[rstest]
    #[case::forwards_and_right(Square::new(Rank::One, File::A), Square::new(Rank::Two, File::B))]
    #[case::back_and_right(Square::new(Rank::Eight, File::A), Square::new(Rank::Two, File::G))]
    #[case::back_and_left(Square::new(Rank::Seven, File::F), Square::new(Rank::Three, File::B))]
    #[case::forwards_and_left(Square::new(Rank::Two, File::H), Square::new(Rank::Seven, File::C))]
    fn allows_bishop_to_move_diagonally(#[case] from_square: Square, #[case] to_square: Square) {
        let bishop = Piece::new(Colour::White, PieceType::Bishop);
        let mut starting_position = BTreeMap::new();
        starting_position.insert(from_square, bishop);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = OrdinaryMove::new(&chessboard, &bishop, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[rstest]
    #[case::forwards(Square::new(Rank::One, File::B), Square::new(Rank::Three, File::B))]
    #[case::sideways(Square::new(Rank::Three, File::C), Square::new(Rank::Three, File::A))]
    #[case::l_shaped(Square::new(Rank::Five, File::E), Square::new(Rank::Seven, File::F))]
    fn disallowed_moves(#[case] from_square: Square, #[case] to_square: Square) {
        let bishop = Piece::new(Colour::White, PieceType::Rook);
        let mut starting_position = BTreeMap::new();
        starting_position.insert(from_square, bishop);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = OrdinaryMove::new(&chessboard, &bishop, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }

    #[test]
    fn disallows_bishop_moving_through_an_obstruction() {
        let from_square = Square::new(Rank::One, File::C);
        let to_square = Square::new(Rank::Four, File::F);
        let bishop = Piece::new(Colour::White, PieceType::Bishop);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &bishop, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }
}
