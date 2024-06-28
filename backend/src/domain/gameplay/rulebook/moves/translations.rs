use crate::domain::gameplay::chess_set;
use std::fmt;
use std::ops;

#[derive(Debug, PartialEq)]
struct ChessVector {
    x: i8,
    y: i8,
}

impl fmt::Display for ChessVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ChessVector {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x: x, y: y }
    }
}

impl ops::Mul<i8> for ChessVector {
    type Output = ChessVector;

    fn mul(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Geometric representation of a chess move, used to simplify validation logic.
#[derive(Debug, PartialEq)]
struct Translation {
    vector: ChessVector,
    scale_factor: u8,
}

pub fn is_move_allowed_for_piece(
    piece: &chess_set::Piece,
    from_square: &chess_set::Square,
    to_square: &chess_set::Square,
) {
}

impl Translation {
    fn new(
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
        piece_colour: &chess_set::Colour,
    ) -> Self {
        let x = to_square.get_file().value() - from_square.get_file().value();
        let y = to_square.get_rank().value() - from_square.get_rank().value();

        // Vectors for black and white are relative to different origins.
        // This is because they use the same references for each square,
        // but are playing in opposite directions.
        let sign = match piece_colour {
            chess_set::Colour::White => 1,
            chess_set::Colour::Black => -1,
        };
        let vector = ChessVector::new(x, y) * sign;

        Self {
            vector: vector,
            scale_factor: 1,
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod translation_new_tests {
        use super::super::{
            chess_set::Colour, chess_set::File, chess_set::Rank, chess_set::Square,
        };
        use crate::domain::gameplay::rulebook::moves::translations::{ChessVector, Translation};
        use rstest::rstest;

        #[rstest]
        #[case::forwards(
            Square::new(Rank::ONE, File::A),
            Square::new(Rank::TWO, File::A),
            ChessVector::new(0, 1)
        )]
        #[case::forwards_and_right(
            Square::new(Rank::THREE, File::C),
            Square::new(Rank::FOUR, File::D),
            ChessVector::new(1, 1)
        )]
        #[case::right(
            Square::new(Rank::SEVEN, File::F),
            Square::new(Rank::SEVEN, File::G),
            ChessVector::new(1, 0)
        )]
        #[case::backwards_and_right(
            Square::new(Rank::TWO, File::B),
            Square::new(Rank::ONE, File::C),
            ChessVector::new(1, -1)
        )]
        #[case::backwards(
            Square::new(Rank::FIVE, File::E),
            Square::new(Rank::FOUR, File::E),
            ChessVector::new(0, -1)
        )]
        #[case::backwards_and_left(
            Square::new(Rank::EIGHT, File::H),
            Square::new(Rank::SEVEN, File::G),
            ChessVector::new(-1, -1)
        )]
        #[case::left(
            Square::new(Rank::FOUR, File::D),
            Square::new(Rank::FOUR, File::C),
            ChessVector::new(-1, 0)
        )]
        #[case::forward_and_left(
            Square::new(Rank::SIX, File::H),
            Square::new(Rank::SEVEN, File::G),
            ChessVector::new(-1, 1)
        )]
        fn single_square_moves_white(
            #[case] from_square: Square,
            #[case] to_square: Square,
            #[case] expected_vector: ChessVector,
        ) {
            let white_translation = Translation::new(&from_square, &to_square, &Colour::White);

            assert_eq!(white_translation.vector, expected_vector);
            assert_eq!(white_translation.scale_factor, 1);

            let black_translation = Translation::new(&from_square, &to_square, &Colour::Black);

            assert_eq!(black_translation.vector, expected_vector * -1);
            assert_eq!(black_translation.scale_factor, 1);
        }

        // TODO
        fn multiple_square_moves_in_a_straight_line() {}

        // TODO
        fn multiple_square_moves_in_a_wonky_line() {}
    }
}
