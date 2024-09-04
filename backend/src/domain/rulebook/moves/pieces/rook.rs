use crate::domain::chess_set;
use crate::domain::rulebook::moves::{chess_move, translation};
use std::vec;

pub fn get_rook_move_rules() -> vec::IntoIter<Box<dyn chess_move::MoveRule>> {
    vec![Box::new(AllowHorizontalOrVertical) as Box<dyn chess_move::MoveRule>].into_iter()
}

struct AllowHorizontalOrVertical;

impl chess_move::MoveRule for AllowHorizontalOrVertical {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let is_valid_vector = rook_allowed_vectors().contains(&chess_move.translation.vector);
        let is_obstructed = chess_move.is_obstructed(chessboard_history.last().unwrap());

        is_valid_vector && !is_obstructed
    }
}

fn rook_allowed_vectors() -> Vec<translation::ChessVector> {
    vec![
        translation::ChessVector::new(0, 1),
        translation::ChessVector::new(1, 0),
        translation::ChessVector::new(0, -1),
        translation::ChessVector::new(-1, 0),
    ]
}

#[cfg(test)]
mod tests {
    use super::get_rook_move_rules;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook::moves::chess_move;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::BTreeMap;

    fn is_move_allowed(chess_move: chess_move::Move, chessboard: &Chessboard) -> bool {
        let mut rules = get_rook_move_rules();
        rules.any(|rule| rule.allows_move(&chess_move, &vec![chessboard.clone()]))
    }

    #[rstest]
    #[case::forwards(Square::new(Rank::One, File::C), Square::new(Rank::Three, File::C))]
    #[case::right(Square::new(Rank::Five, File::A), Square::new(Rank::Five, File::H))]
    #[case::backwards(Square::new(Rank::Seven, File::F), Square::new(Rank::Two, File::F))]
    #[case::left(Square::new(Rank::Three, File::G), Square::new(Rank::Three, File::F))]
    fn allows_rook_to_move_within_plus(#[case] from_square: Square, #[case] to_square: Square) {
        let rook = Piece::new(Colour::White, PieceType::Rook);
        let mut starting_position = BTreeMap::new();
        starting_position.insert(from_square, rook);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = chess_move::Move::new(rook, from_square, to_square);

        assert!(is_move_allowed(chess_move, &chessboard));
    }

    #[rstest]
    #[case::diagonal(Square::new(Rank::One, File::B), Square::new(Rank::Two, File::C))]
    #[case::l_shaped(Square::new(Rank::Five, File::E), Square::new(Rank::Seven, File::F))]
    fn disallowed_moves(#[case] from_square: Square, #[case] to_square: Square) {
        let rook = Piece::new(Colour::White, PieceType::Rook);
        let mut starting_position = BTreeMap::new();
        starting_position.insert(from_square, rook);

        let chessboard = Chessboard::new(starting_position);
        let chess_move = chess_move::Move::new(rook, from_square, to_square);

        assert!(!is_move_allowed(chess_move, &chessboard));
    }

    #[test]
    fn disallows_rook_moving_through_an_obstruction() {
        let from_square = Square::new(Rank::One, File::H);
        let to_square = Square::new(Rank::Five, File::H);
        let rook = Piece::new(Colour::White, PieceType::Rook);

        let chessboard = factories::chessboard();
        let chess_move = chess_move::Move::new(rook, from_square, to_square);

        assert!(!is_move_allowed(chess_move, &chessboard));
    }
}
