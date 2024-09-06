use crate::domain::chess_set;
use crate::domain::rulebook::check;
use crate::domain::rulebook::moves::chess_move;
use std::collections::BTreeMap;

pub struct AllowCastle;

impl chess_move::MoveRule for AllowCastle {
    fn allows_move(
        &self,
        chess_move: &chess_move::Move,
        chessboard_history: &Vec<chess_set::Chessboard>,
    ) -> bool {
        let chessboard = chessboard_history.last().unwrap();

        // Check the king's move is valid.
        if !is_king_moving_from_valid_square(chess_move) {
            return false;
        };
        if !is_king_moving_to_valid_square(chess_move) {
            return false;
        };
        if has_piece_at_square_previously_moved(chess_move, chessboard_history) {
            return false;
        };
        if chess_move.is_obstructed(chessboard) {
            return false;
        };

        // Check the corresponding rook move is valid.
        let rook_move = get_corresponding_legal_rook_move(chess_move);
        if has_piece_at_square_previously_moved(&rook_move, chessboard_history) {
            return false;
        }
        if rook_move.is_obstructed(chessboard) {
            return false;
        };

        // Check the king isn't moving out of or through check.
        if check::is_player_in_check(chess_move.piece.get_colour(), chessboard.clone()) {
            return false;
        }

        true
    }

    fn get_move_outcome(
        &self,
        chess_move: &chess_move::Move,
    ) -> BTreeMap<chess_set::Square, Option<chess_set::Piece>> {
        let mut outcome = BTreeMap::new();
        outcome.insert(chess_move.from_square, None);
        outcome.insert(chess_move.to_square, Some(chess_move.piece));

        let rook_move = get_corresponding_legal_rook_move(chess_move);
        outcome.insert(rook_move.from_square, None);
        outcome.insert(rook_move.to_square, Some(rook_move.piece));

        outcome
    }
}

// King move helpers.

fn is_king_moving_from_valid_square(chess_move: &chess_move::Move) -> bool {
    let valid_file = chess_move.from_square.get_file() == &chess_set::File::E;

    let actual_rank = chess_move.from_square.get_rank();
    let valid_rank = match chess_move.piece.get_colour() {
        chess_set::Colour::White => actual_rank == &chess_set::Rank::One,
        chess_set::Colour::Black => actual_rank == &chess_set::Rank::Eight,
    };

    valid_rank && valid_file
}

fn is_king_moving_to_valid_square(chess_move: &chess_move::Move) -> bool {
    let valid_rank = chess_move.to_square.get_rank() == chess_move.from_square.get_rank();

    let actual_file = chess_move.to_square.get_file();
    let valid_file = actual_file == &chess_set::File::C || actual_file == &chess_set::File::G;

    valid_rank && valid_file
}

fn has_piece_at_square_previously_moved(
    chess_move: &chess_move::Move,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> bool {
    for chessboard in chessboard_history.into_iter() {
        let piece = chessboard.get_piece(&chess_move.from_square);
        if !(piece == Some(chess_move.piece)) {
            return true;
        }
    }
    false
}

// Rook move helpers.

fn get_corresponding_legal_rook_move(king_move: &chess_move::Move) -> chess_move::Move {
    let rank = king_move.from_square.get_rank();

    let from_file = match king_move.to_square.get_file() {
        chess_set::File::C => chess_set::File::A, // Queenside castle.
        chess_set::File::G => chess_set::File::H, // Kingside castle.
        _ => panic!("King's move should be validated first!"),
    };

    let to_file = match king_move.to_square.get_file() {
        chess_set::File::C => chess_set::File::D, // Queenside castle.
        chess_set::File::G => chess_set::File::F, // Kingside castle.
        _ => return panic!("King's move should be validated first!"),
    };

    let from_square = chess_set::Square::new(rank.clone(), from_file.clone());
    let to_square = chess_set::Square::new(rank.clone(), to_file.clone());
    let rook = chess_set::Piece::new(
        king_move.piece.get_colour().clone(),
        chess_set::PieceType::Rook,
    );

    chess_move::Move::new(rook, from_square, to_square)
}

#[cfg(test)]
mod tests {
    use super::AllowCastle;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::rulebook::moves::chess_move;
    use crate::domain::rulebook::moves::chess_move::MoveRule;
    use rstest::rstest;
    use std::collections::BTreeMap;

    #[rstest]
    #[case::queenside(File::C, File::A)]
    #[case::kingside(File::G, File::H)]
    fn white_king_can_castle(#[case] king_to_file: File, #[case] rook_from_file: File) {
        let mut starting_position = BTreeMap::new();

        let king_from_square = Square::new(Rank::One, File::E);
        let white_king = Piece::new(Colour::White, PieceType::King);
        starting_position.insert(king_from_square, white_king);

        let rook_from_square = Square::new(Rank::One, rook_from_file);
        let white_rook = Piece::new(Colour::White, PieceType::Rook);
        starting_position.insert(rook_from_square, white_rook);

        let chessboard = Chessboard::new(starting_position);

        let king_to_square = Square::new(Rank::One, king_to_file);
        let castle = chess_move::Move::new(white_king, king_from_square, king_to_square);

        assert!(AllowCastle.allows_move(&castle, &vec![chessboard]));

        let outcome = AllowCastle.get_move_outcome(&castle);
        assert_eq!(outcome.get(&king_from_square).unwrap(), &None);
        assert_eq!(outcome.get(&rook_from_square).unwrap(), &None);
        assert_eq!(outcome.get(&king_to_square).unwrap(), &Some(white_king));
    }

    #[rstest]
    #[case::queenside(File::C, File::A)]
    #[case::kingside(File::G, File::H)]
    fn black_king_can_castle(#[case] king_to_file: File, #[case] rook_from_file: File) {
        let mut starting_position = BTreeMap::new();

        let king_from_square = Square::new(Rank::Eight, File::E);
        let black_king = Piece::new(Colour::Black, PieceType::King);
        starting_position.insert(king_from_square, black_king);

        let rook_from_square = Square::new(Rank::Eight, rook_from_file);
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        starting_position.insert(rook_from_square, black_rook);

        let chessboard = Chessboard::new(starting_position);

        let king_to_square = Square::new(Rank::Eight, king_to_file);
        let castle = chess_move::Move::new(black_king, king_from_square, king_to_square);

        assert!(AllowCastle.allows_move(&castle, &vec![chessboard]));
    }

    #[test]
    fn castle_disallowed_if_player_is_in_check() {
        let mut starting_position = BTreeMap::new();

        let king_from_square = Square::new(Rank::One, File::E);
        let white_king = Piece::new(Colour::White, PieceType::King);
        starting_position.insert(king_from_square, white_king);

        let rook_from_square = Square::new(Rank::One, File::A);
        let white_rook = Piece::new(Colour::White, PieceType::Rook);
        starting_position.insert(rook_from_square, white_rook);

        let black_queen_square = Square::new(Rank::Three, File::G);
        let white_rook = Piece::new(Colour::Black, PieceType::Queen);
        starting_position.insert(black_queen_square, white_rook);

        let chessboard = Chessboard::new(starting_position);

        let king_to_square = Square::new(Rank::One, File::C);
        let castle = chess_move::Move::new(white_king, king_from_square, king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &vec![chessboard]));
    }

    #[test]
    fn castle_disallowed_if_king_not_on_starting_square() {
        let mut starting_position = BTreeMap::new();

        let invalid_king_from_square = Square::new(Rank::One, File::D);
        let white_king = Piece::new(Colour::White, PieceType::King);
        starting_position.insert(invalid_king_from_square, white_king);

        let rook_from_square = Square::new(Rank::One, File::A);
        let white_rook = Piece::new(Colour::White, PieceType::Rook);
        starting_position.insert(rook_from_square, white_rook);

        let chessboard = Chessboard::new(starting_position);

        let king_to_square = Square::new(Rank::One, File::C);
        let castle = chess_move::Move::new(white_king, invalid_king_from_square, king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &vec![chessboard]));
    }

    #[test]
    fn castle_disallowed_if_king_not_moving_to_valid_square() {
        let mut starting_position = BTreeMap::new();

        let king_from_square = Square::new(Rank::Eight, File::E);
        let black_king = Piece::new(Colour::Black, PieceType::King);
        starting_position.insert(king_from_square, black_king);

        let rook_from_square = Square::new(Rank::Eight, File::A);
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        starting_position.insert(rook_from_square, black_rook);

        let chessboard = Chessboard::new(starting_position);

        let invalid_king_to_square = Square::new(Rank::Eight, File::D);
        let castle = chess_move::Move::new(black_king, king_from_square, invalid_king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &vec![chessboard]));
    }

    #[test]
    fn castle_disallowed_if_king_has_already_moved() {
        let mut previous_position = BTreeMap::new();

        let invalid_king_from_square = Square::new(Rank::Eight, File::D);
        let black_king = Piece::new(Colour::Black, PieceType::King);
        previous_position.insert(invalid_king_from_square, black_king.clone());

        let rook_from_square = Square::new(Rank::Eight, File::A);
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        previous_position.insert(rook_from_square, black_rook);

        let previous_chessboard = Chessboard::new(previous_position);
        let mut current_chessboard = previous_chessboard.clone();

        let mut position_updates = BTreeMap::new();
        let valid_king_from_square = Square::new(Rank::Eight, File::E);
        position_updates.insert(invalid_king_from_square, None);
        position_updates.insert(valid_king_from_square, Some(black_king));

        current_chessboard.update_position(position_updates);
        let chessboard_history = vec![previous_chessboard, current_chessboard];

        let valid_king_to_square = Square::new(Rank::Eight, File::C);
        let castle =
            chess_move::Move::new(black_king, valid_king_from_square, valid_king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &chessboard_history));
    }

    #[test]
    fn castle_disallowed_if_obstructed() {
        let mut starting_position = BTreeMap::new();

        let king_from_square = Square::new(Rank::One, File::E);
        let white_king = Piece::new(Colour::White, PieceType::King);
        starting_position.insert(king_from_square, white_king);

        let rook_from_square = Square::new(Rank::One, File::A);
        let white_rook = Piece::new(Colour::White, PieceType::Rook);
        starting_position.insert(rook_from_square, white_rook);

        let blocking_knight_square = Square::new(Rank::One, File::D);
        let white_rook = Piece::new(Colour::White, PieceType::Knight);
        starting_position.insert(blocking_knight_square, white_rook);

        let chessboard = Chessboard::new(starting_position);

        let king_to_square = Square::new(Rank::One, File::C);
        let castle = chess_move::Move::new(white_king, king_from_square, king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &vec![chessboard]));
    }

    #[test]
    fn castle_disallowed_if_rook_not_in_valid_position() {
        let mut starting_position = BTreeMap::new();

        let king_from_square = Square::new(Rank::One, File::E);
        let white_king = Piece::new(Colour::White, PieceType::King);
        starting_position.insert(king_from_square, white_king);

        let chessboard = Chessboard::new(starting_position);

        let king_to_square = Square::new(Rank::One, File::C);
        let castle = chess_move::Move::new(white_king, king_from_square, king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &vec![chessboard]));
    }

    #[test]
    fn castle_disallowed_if_rook_has_already_moved() {
        let mut previous_position = BTreeMap::new();

        let valid_king_from_square = Square::new(Rank::Eight, File::E);
        let black_king = Piece::new(Colour::Black, PieceType::King);
        previous_position.insert(valid_king_from_square, black_king);

        let invalid_rook_from_square = Square::new(Rank::Eight, File::B);
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        previous_position.insert(invalid_rook_from_square, black_rook.clone());

        let previous_chessboard = Chessboard::new(previous_position);
        let mut current_chessboard = previous_chessboard.clone();

        let mut position_updates = BTreeMap::new();
        let valid_rook_from_square = Square::new(Rank::Eight, File::A);
        position_updates.insert(invalid_rook_from_square, None);
        position_updates.insert(valid_rook_from_square, Some(black_rook));

        current_chessboard.update_position(position_updates);
        let chessboard_history = vec![previous_chessboard, current_chessboard];

        let valid_king_to_square = Square::new(Rank::Eight, File::C);
        let castle =
            chess_move::Move::new(black_king, valid_king_from_square, valid_king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &chessboard_history));
    }

    #[test]
    fn castle_disallowed_if_rook_is_obstructed() {
        let mut starting_position = BTreeMap::new();

        let king_from_square = Square::new(Rank::Eight, File::E);
        let black_king = Piece::new(Colour::Black, PieceType::King);
        starting_position.insert(king_from_square, black_king);

        let rook_from_square = Square::new(Rank::Eight, File::A);
        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        starting_position.insert(rook_from_square, black_rook);

        let blocking_knight_square = Square::new(Rank::Eight, File::B);
        let white_rook = Piece::new(Colour::Black, PieceType::Knight);
        starting_position.insert(blocking_knight_square, white_rook);

        let chessboard = Chessboard::new(starting_position);

        let king_to_square = Square::new(Rank::Eight, File::C);
        let castle = chess_move::Move::new(black_king, king_from_square, king_to_square);

        assert!(!AllowCastle.allows_move(&castle, &vec![chessboard]));
    }
}
