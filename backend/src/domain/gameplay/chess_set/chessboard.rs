use super::piece;
use super::square;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use thiserror;

/// Representation of a physical chessboard, and the current position of all pieces.
///
/// Note: this does not implement any gameplay logic or rules of the game.
/// The only invariant enforced is that each square has at most one piece on it
/// at any point in time (since the chessboard is represented by a hashmap).
#[derive(Clone, Debug)]
pub struct Chessboard {
    pub position: HashMap<square::Square, Option<piece::Piece>>,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ChessboardActionError {
    #[error("{0} is empty.")]
    SquareIsEmpty(square::Square),

    #[error("{1} is not empty - it contains {0}!.")]
    SquareIsNotEmpty(piece::Piece, square::Square),
}

impl Chessboard {
    // Factories

    pub fn new(starting_position: HashMap<square::Square, piece::Piece>) -> Self {
        let mut position = HashMap::new();

        // Initialise an empty board.
        for rank in square::Rank::iter() {
            for file in square::File::iter() {
                let square = square::Square::new(rank, file);
                position.insert(square, None);
            }
        }

        // Extract the starting position.
        for (square, piece) in starting_position.into_iter() {
            position.insert(square, Some(piece));
        }

        Chessboard { position: position }
    }

    // Queries

    pub fn get_piece(&self, square: &square::Square) -> Option<piece::Piece> {
        self.position.get(square).unwrap().clone()
    }

    pub fn get_pieces(&self, colour: piece::Colour) -> HashMap<square::Square, piece::Piece> {
        let mut pieces = HashMap::new();
        for (square, maybe_piece) in self.position.clone() {
            let Some(piece) = maybe_piece else { continue };
            if piece.get_colour() == &colour {
                pieces.insert(square, piece);
            }
        }
        pieces
    }

    pub fn get_square_king_is_on(&self, colour: &piece::Colour) -> square::Square {
        for (square, maybe_piece) in self.position.clone().into_iter() {
            let Some(piece) = maybe_piece else { continue };
            if piece.get_colour() == colour && piece.get_piece_type() == &piece::PieceType::King {
                return square;
            }
        }

        panic!("No {} king on chessboard!", colour)
    }

    pub fn is_square_occupied(&self, square: &square::Square) -> bool {
        if let Some(_) = self.position.get(square).unwrap() {
            true
        } else {
            false
        }
    }

    // Mutators
    pub fn move_piece(
        &mut self,
        from_square: &square::Square,
        to_square: &square::Square,
    ) -> Result<(), ChessboardActionError> {
        let Ok(piece) = self.remove_piece(from_square) else {
            return Err(ChessboardActionError::SquareIsEmpty(from_square.clone()));
        };
        let _ = self.remove_piece(to_square);
        let _ = self.add_piece(piece, to_square);
        Ok(())
    }

    pub fn add_piece(
        &mut self,
        piece: piece::Piece,
        to_square: &square::Square,
    ) -> Result<(), ChessboardActionError> {
        if let Some(piece) = self.get_piece(to_square) {
            return Err(ChessboardActionError::SquareIsNotEmpty(
                piece.clone(),
                to_square.clone(),
            ));
        }
        self.position.insert(to_square.clone(), Some(piece));
        Ok(())
    }

    pub fn remove_piece(
        &mut self,
        from_square: &square::Square,
    ) -> Result<piece::Piece, ChessboardActionError> {
        let Some(piece) = self.get_piece(from_square) else {
            return Err(ChessboardActionError::SquareIsEmpty(from_square.clone()));
        };
        self.position.insert(from_square.clone(), None); // Empty the square.
        Ok(piece)
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut representation = "--|---- ---- ---- ---- ---- ---- ---- ----|".to_string();

        for rank in square::Rank::iter().rev() {
            let mut rank_repr = format!("\n{} |", rank.index());

            for file in square::File::iter() {
                let square = square::Square::new(rank, file);
                match self.get_piece(&square) {
                    Some(piece) => rank_repr.push_str(format!(" {} ", piece).as_str()),
                    None => rank_repr.push_str("    "),
                }
                rank_repr.push_str("|");
            }

            representation.push_str(rank_repr.as_str());
            representation.push_str("\n  |---- ---- ---- ---- ---- ---- ---- ----|");
        }

        representation.push_str("\n--| A    B    C    D    E    F    G    H  |");
        write!(f, "{}", representation)
    }
}


#[cfg(test)]
mod tests {
    use crate::testing::factories;

    #[test]
    fn formats_board_in_a_useful_way() {
        let chessboard = factories::chessboard();

        let formatted_chessboard = format!("{}", chessboard);

        let expected = "\
--|---- ---- ---- ---- ---- ---- ---- ----|
8 | BR | BN | BB | BQ | BK | BB | BN | BR |
  |---- ---- ---- ---- ---- ---- ---- ----|
7 | BP | BP | BP | BP | BP | BP | BP | BP |
  |---- ---- ---- ---- ---- ---- ---- ----|
6 |    |    |    |    |    |    |    |    |
  |---- ---- ---- ---- ---- ---- ---- ----|
5 |    |    |    |    |    |    |    |    |
  |---- ---- ---- ---- ---- ---- ---- ----|
4 |    |    |    |    |    |    |    |    |
  |---- ---- ---- ---- ---- ---- ---- ----|
3 |    |    |    |    |    |    |    |    |
  |---- ---- ---- ---- ---- ---- ---- ----|
2 | WP | WP | WP | WP | WP | WP | WP | WP |
  |---- ---- ---- ---- ---- ---- ---- ----|
1 | WR | WN | WB | WQ | WK | WB | WN | WR |
  |---- ---- ---- ---- ---- ---- ---- ----|
--| A    B    C    D    E    F    G    H  |";

        assert_eq!(formatted_chessboard, expected)
    }

    #[cfg(test)]
    mod new_tests {
        use super::super::*;
        use crate::testing::factories;

        #[test]
        fn can_create_new_board() {
            let mut starting_position = HashMap::new();

            let square = factories::some_square();
            let piece = factories::some_piece();
            starting_position.insert(square, piece);

            let other_square = factories::some_other_square();

            let chessboard = Chessboard::new(starting_position);

            assert_eq!(chessboard.get_piece(&square), Some(piece));
            assert_eq!(chessboard.get_piece(&other_square), None);
        }
    }

    #[cfg(test)]
    mod get_pieces_tests {
        use super::super::*;
        use crate::testing::factories;

        #[test]
        fn gets_black_pieces() {
            let mut starting_position = HashMap::new();

            let black_square = factories::some_square();
            let black_king = piece::Piece::new(piece::Colour::Black, piece::PieceType::King);
            starting_position.insert(black_square, black_king);

            let white_square = factories::some_other_square();
            let white_pawn = piece::Piece::new(piece::Colour::White, piece::PieceType::Pawn);
            starting_position.insert(white_square, white_pawn);

            let chessboard = Chessboard::new(starting_position);
            let black_pieces = chessboard.get_pieces(piece::Colour::Black);

            assert_eq!(black_pieces.get(&black_square), Some(&black_king));
            assert_eq!(black_pieces.get(&white_square), None);
        }
    }

    #[cfg(test)]
    mod get_square_king_is_on_tests {
        use super::super::*;
        use crate::testing::factories;

        #[test]
        fn gets_starting_square_for_white_king() {
            let chessboard = factories::chessboard();

            let king_square = chessboard.get_square_king_is_on(&piece::Colour::White);

            assert_eq!(king_square.get_rank(), &square::Rank::One);
            assert_eq!(king_square.get_file(), &square::File::E);
        }

        #[test]
        fn gets_starting_square_for_black_king() {
            let chessboard = factories::chessboard();

            let king_square = chessboard.get_square_king_is_on(&piece::Colour::Black);

            assert_eq!(king_square.get_rank(), &square::Rank::Eight);
            assert_eq!(king_square.get_file(), &square::File::E);
        }

        #[should_panic(expected = "No W king on chessboard!")]
        #[test]
        fn panics_when_no_king_matching_colour_is_on_board() {
            let starting_position = HashMap::new();
            let chessboard = Chessboard::new(starting_position);

            let _ = chessboard.get_square_king_is_on(&piece::Colour::White);
        }
    }

    #[cfg(test)]
    mod is_square_occupied_tests {
        use super::super::Chessboard;
        use crate::testing::factories;
        use std::collections::HashMap;

        #[test]
        fn is_occupied() {
            let mut starting_position = HashMap::new();

            let square = factories::some_square();
            let piece = factories::some_piece();
            starting_position.insert(square, piece);

            let chessboard = Chessboard::new(starting_position);

            assert!(chessboard.is_square_occupied(&square));
        }

        #[test]
        fn is_not_occupied() {
            let starting_position = HashMap::new();
            let chessboard = Chessboard::new(starting_position);

            let square = factories::some_square();

            assert!(!chessboard.is_square_occupied(&square));
        }
    }

    #[cfg(test)]
    mod move_piece_tests {
        use super::super::*;
        use crate::testing::factories;

        #[test]
        fn can_move_piece_to_empty_square() {
            let mut starting_position = HashMap::new();

            let from_square = factories::some_square();
            let piece = factories::some_piece();
            starting_position.insert(from_square, piece);

            let to_square = factories::some_other_square();

            let mut chessboard = Chessboard::new(starting_position);

            let result = chessboard.move_piece(&from_square, &to_square);

            assert_eq!(result, Ok(()));
            assert_eq!(chessboard.get_piece(&from_square), None);
            assert_eq!(chessboard.get_piece(&to_square), Some(piece));
        }

        #[test]
        fn can_move_piece_to_occupied_square() {
            let mut starting_position = HashMap::new();

            let from_square = factories::some_square();
            let piece = factories::some_piece();
            starting_position.insert(from_square, piece);

            let to_square = factories::some_other_square();
            let other_piece = factories::some_other_piece();
            starting_position.insert(to_square, other_piece);

            let mut chessboard = Chessboard::new(starting_position);

            let result = chessboard.move_piece(&from_square, &to_square);

            assert_eq!(result, Ok(()));
            assert_eq!(chessboard.get_piece(&from_square), None);
            assert_eq!(chessboard.get_piece(&to_square), Some(piece));
        }

        #[test]
        fn error_when_moving_from_empty_square() {
            let starting_position = HashMap::new();
            let mut chessboard = Chessboard::new(starting_position);

            let from_square = factories::some_square();
            let to_square = factories::some_square();

            let result = chessboard.move_piece(&from_square, &to_square);

            assert_eq!(
                result,
                Err(ChessboardActionError::SquareIsEmpty(from_square))
            );
            assert_eq!(chessboard.get_piece(&from_square), None);
            assert_eq!(chessboard.get_piece(&to_square), None);
        }
    }

    #[cfg(test)]
    mod add_piece_tests {
        use super::super::*;
        use crate::testing::factories;

        #[test]
        fn can_add_piece_to_empty_square() {
            let starting_position = HashMap::new();
            let mut chessboard = Chessboard::new(starting_position);

            let square = factories::some_square();
            let piece = factories::some_piece();

            let result = chessboard.add_piece(piece, &square);

            assert_eq!(result, Ok(()));
            assert_eq!(chessboard.get_piece(&square), Some(piece));
        }

        #[test]
        fn error_when_adding_piece_to_occupied_square() {
            let mut starting_position = HashMap::new();

            let square = factories::some_square();
            let piece = factories::some_piece();
            starting_position.insert(square, piece);

            let mut chessboard = Chessboard::new(starting_position);
            let other_piece = factories::some_other_piece();

            let result = chessboard.add_piece(other_piece, &square);

            let expected_err = Err(ChessboardActionError::SquareIsNotEmpty(piece, square));
            assert_eq!(result, expected_err);
            assert_eq!(chessboard.get_piece(&square), Some(piece));
        }
    }
}
