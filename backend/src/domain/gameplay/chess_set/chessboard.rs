use crate::domain::gameplay::chess_set;
use std::collections::HashMap;
use thiserror;

/// Representation of a physical chessboard, and the current position of all ordinary_move.
///
/// Note: this does not implement any gameplay logic or rules of the game.
/// The only invariant enforced is that each square has at most one piece on it
/// at any point in time (since the chessboard is represented by a hashmap).
#[derive(Clone)]
pub struct Chessboard {
    position: HashMap<chess_set::Square, Option<chess_set::Piece>>,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ChessboardActionError {
    #[error("{0} is empty.")]
    SquareIsEmpty(chess_set::Square),

    #[error("{1} is not empty - it contains {0}!.")]
    SquareIsNotEmpty(chess_set::Piece, chess_set::Square),
}

impl Chessboard {
    // Factories

    pub fn new(starting_position: HashMap<chess_set::Square, chess_set::Piece>) -> Self {
        let mut position = HashMap::new();

        // Initialise an empty board.
        for rank in chess_set::Rank::iter() {
            for file in chess_set::File::iter() {
                let square = chess_set::Square::new(rank, file);
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

    pub fn get_piece(&self, square: &chess_set::Square) -> Option<chess_set::Piece> {
        self.position.get(square).unwrap().clone()
    }

    pub fn get_pieces(
        &self,
        colour: chess_set::Colour,
    ) -> HashMap<chess_set::Square, chess_set::Piece> {
        let mut pieces = HashMap::new();
        for (square, maybe_piece) in self.position.clone() {
            let Some(piece) = maybe_piece else { continue };
            if piece.get_colour() == &colour {
                pieces.insert(square, piece);
            }
        }
        pieces
    }

    pub fn get_square_king_is_on(&self, colour: chess_set::Colour) -> chess_set::Square {
        for (square, maybe_piece) in self.position.clone().into_iter() {
            let Some(piece) = maybe_piece else { continue };
            if piece.get_colour() == &colour
                && piece.get_piece_type() == &chess_set::PieceType::King
            {
                return square;
            }
        }

        panic!("No {} king on chessboard!", colour)
    }

    // Mutators
    pub fn move_piece(
        &mut self,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
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
        piece: chess_set::Piece,
        to_square: &chess_set::Square,
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
        from_square: &chess_set::Square,
    ) -> Result<chess_set::Piece, ChessboardActionError> {
        let Some(piece) = self.get_piece(from_square) else {
            return Err(ChessboardActionError::SquareIsEmpty(from_square.clone()));
        };
        self.position.insert(from_square.clone(), None); // Empty the square.
        Ok(piece)
    }
}

#[cfg(test)]
mod tests {

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
            let black_king =
                chess_set::Piece::new(chess_set::Colour::Black, chess_set::PieceType::King);
            starting_position.insert(black_square, black_king);

            let white_square = factories::some_other_square();
            let white_pawn =
                chess_set::Piece::new(chess_set::Colour::White, chess_set::PieceType::Pawn);
            starting_position.insert(white_square, white_pawn);

            let mut chessboard = Chessboard::new(starting_position);
            let black_pieces = chessboard.get_pieces(chess_set::Colour::Black);

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
            let mut chessboard = factories::chessboard();

            let king_square = chessboard.get_square_king_is_on(chess_set::Colour::White);

            assert_eq!(king_square.get_rank(), &chess_set::Rank::One);
            assert_eq!(king_square.get_file(), &chess_set::File::E);
        }

        #[test]
        fn gets_starting_square_for_black_king() {
            let mut chessboard = factories::chessboard();

            let king_square = chessboard.get_square_king_is_on(chess_set::Colour::Black);

            assert_eq!(king_square.get_rank(), &chess_set::Rank::Eight);
            assert_eq!(king_square.get_file(), &chess_set::File::E);
        }

        #[should_panic(expected = "No White king on chessboard!")]
        #[test]
        fn panics_when_no_king_matching_colour_is_on_board() {
            let starting_position = HashMap::new();
            let mut chessboard = Chessboard::new(starting_position);

            let king_square = chessboard.get_square_king_is_on(chess_set::Colour::White);
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
