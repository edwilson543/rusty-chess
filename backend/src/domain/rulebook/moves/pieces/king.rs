use crate::domain::chess_set;
use crate::domain::rulebook::moves::{chess_move, translation};
use std::vec;

pub fn get_king_move_rules() -> vec::IntoIter<Box<dyn chess_move::MoveRule>> {
    vec![Box::new(AllowSingleSquareAnyDirection) as Box<dyn chess_move::MoveRule>].into_iter()
}

struct AllowSingleSquareAnyDirection;

impl chess_move::MoveRule for AllowSingleSquareAnyDirection {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let _ = chessboard_history;

        let is_valid_vector = king_allowed_vectors().contains(&chess_move.translation.vector);
        let is_single_square = chess_move.translation.scalar == 1;

        is_valid_vector && is_single_square
    }
}

fn king_allowed_vectors() -> Vec<translation::ChessVector> {
    vec![
        translation::ChessVector::new(0, 1),
        translation::ChessVector::new(1, 1),
        translation::ChessVector::new(1, 0),
        translation::ChessVector::new(1, -1),
        translation::ChessVector::new(0, -1),
        translation::ChessVector::new(-1, -1),
        translation::ChessVector::new(-1, 0),
        translation::ChessVector::new(-1, 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook::moves::chess_move;
    use crate::testing::factories;
    use rstest::rstest;

    fn is_move_allowed(chess_move: chess_move::Move, chessboard: &Chessboard) -> bool {
        let mut rules = get_king_move_rules();
        rules.any(|rule| rule.allows_move(&chess_move, &vec![chessboard.clone()]))
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
    fn allows_king_to_move_single_square_in_any_direction(
        #[case] from_square: Square,
        #[case] to_square: Square,
    ) {
        let king = Piece::new(Colour::White, PieceType::King);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(king, from_square, to_square);

        assert!(is_move_allowed(chess_move, &chessboard));
    }

    #[rstest]
    #[case::forwards_multiple(Square::new(Rank::One, File::B), Square::new(Rank::Three, File::B))]
    #[case::sideways_multiple(Square::new(Rank::Three, File::C), Square::new(Rank::Three, File::G))]
    #[case::l_shaped(Square::new(Rank::Five, File::E), Square::new(Rank::Seven, File::F))]
    fn disallowed_moves(#[case] from_square: Square, #[case] to_square: Square) {
        let king = Piece::new(Colour::White, PieceType::King);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(king, from_square, to_square);

        assert!(!is_move_allowed(chess_move, &chessboard));
    }
}
