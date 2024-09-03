use crate::domain::chess_set;
use crate::domain::rulebook::moves_v2::{chess_move, translation};
use std::vec;

pub fn get_knight_move_rules() -> vec::IntoIter<Box<dyn chess_move::MoveRule>> {
    vec![Box::new(AllowLShapedJumped) as Box<dyn chess_move::MoveRule>].into_iter()
}

struct AllowLShapedJumped;

impl chess_move::MoveRule for AllowLShapedJumped {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let _ = chessboard_history;

        let is_valid_vector = knight_allowed_vectors().contains(&chess_move.translation.vector);
        let is_single_jump = chess_move.translation.scalar == 1;

        is_valid_vector && is_single_jump
    }
}

fn knight_allowed_vectors() -> Vec<translation::ChessVector> {
    vec![
        translation::ChessVector::new(1, 2),
        translation::ChessVector::new(2, 1),
        translation::ChessVector::new(2, -1),
        translation::ChessVector::new(1, -2),
        translation::ChessVector::new(-1, -2),
        translation::ChessVector::new(-2, -1),
        translation::ChessVector::new(-2, 1),
        translation::ChessVector::new(-1, 2),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook::moves_v2::chess_move;
    use crate::testing::factories;
    use rstest::rstest;

    fn is_move_allowed(chess_move: chess_move::Move, chessboard: &Chessboard) -> bool {
        let mut rules = get_knight_move_rules();
        rules.any(|rule| rule.allows_move(&chess_move, &vec![chessboard.clone()]))
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
        let chess_move = chess_move::Move::new(knight, from_square, to_square);

        assert!(is_move_allowed(chess_move, &chessboard));
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
        let chess_move = chess_move::Move::new(knight, from_square, to_square);

        assert!(is_move_allowed(chess_move, &chessboard));
    }

    #[rstest]
    #[case::forwards(Square::new(Rank::One, File::B), Square::new(Rank::Three, File::B))]
    #[case::sideways(Square::new(Rank::Three, File::C), Square::new(Rank::Three, File::A))]
    #[case::diagonal(Square::new(Rank::Five, File::E), Square::new(Rank::Seven, File::G))]
    fn disallowed_moves(#[case] from_square: Square, #[case] to_square: Square) {
        let knight = Piece::new(Colour::White, PieceType::Knight);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(knight, from_square, to_square);

        assert!(!is_move_allowed(chess_move, &chessboard));
    }
}
