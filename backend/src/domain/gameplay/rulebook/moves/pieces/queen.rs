use super::{bishop, rook};
use crate::domain::gameplay::rulebook::moves::ordinary_move;
use std::vec;

pub fn get_queen_move_rules() -> vec::IntoIter<Box<dyn ordinary_move::OrdinaryMoveRule>> {
    let mut queen_rules: Vec<Box<dyn ordinary_move::OrdinaryMoveRule>> = vec![];
    for bishop_rule in bishop::get_bishop_move_rules() {
        queen_rules.push(bishop_rule);
    }
    for rook_rule in rook::get_rook_move_rules() {
        queen_rules.push(rook_rule);
    }
    queen_rules.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::gameplay::rulebook::moves::ordinary_move::OrdinaryMove;
    use crate::testing::factories;
    use rstest::rstest;

    fn is_move_allowed(chess_move: &OrdinaryMove) -> bool {
        let mut rules = get_queen_move_rules();
        rules.any(|rule| rule.allows_move(chess_move))
    }

    #[rstest]
    // Verticals.
    #[case::forwards(Square::new(Rank::One, File::C), Square::new(Rank::Three, File::C))]
    #[case::backwards(Square::new(Rank::Seven, File::F), Square::new(Rank::Two, File::F))]
    // Horizontals.
    #[case::right(Square::new(Rank::Five, File::A), Square::new(Rank::Five, File::H))]
    #[case::left(Square::new(Rank::Three, File::G), Square::new(Rank::Three, File::F))]
    // Diagonals.
    #[case::forwards_and_right(Square::new(Rank::One, File::A), Square::new(Rank::Two, File::B))]
    #[case::back_and_right(Square::new(Rank::Eight, File::A), Square::new(Rank::Two, File::G))]
    #[case::back_and_left(Square::new(Rank::Seven, File::F), Square::new(Rank::Three, File::B))]
    #[case::forwards_and_left(Square::new(Rank::Two, File::H), Square::new(Rank::Seven, File::C))]
    fn allows_queen_to_move_straight_line_in_any_direction(
        #[case] from_square: Square,
        #[case] to_square: Square,
    ) {
        let queen = Piece::new(Colour::White, PieceType::Queen);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &queen, &from_square, &to_square);

        assert!(is_move_allowed(&chess_move));
    }

    #[test]
    fn disallows_queen_moving_in_an_l_shape() {
        let from_square = Square::new(Rank::Five, File::E);
        let to_square = Square::new(Rank::Seven, File::F);
        let queen = Piece::new(Colour::White, PieceType::Rook);

        let chessboard = factories::chessboard();
        let chess_move = OrdinaryMove::new(&chessboard, &queen, &from_square, &to_square);

        assert!(!is_move_allowed(&chess_move));
    }
}
