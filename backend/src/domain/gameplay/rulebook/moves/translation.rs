use crate::domain::gameplay::chess_set;
use std::cmp;
use std::fmt;
use std::ops;

/// Geometric representation of a chess move, used to simplify validation logic.
/// /// Note that translations are colour agnostic - i.e.
#[derive(Clone, Debug, PartialEq)]
pub struct Translation {
    pub vector: ChessVector,
    pub scalar: u8,
}

impl Translation {
    // Factories.
    pub fn from_move(from_square: &chess_set::Square, to_square: &chess_set::Square) -> Self {
        let x_unscaled = to_square.get_file().index() - from_square.get_file().index();
        let y_unscaled = to_square.get_rank().index() - from_square.get_rank().index();

        // max(gcd, 1) is taken here to avoid dividing by zero.
        let scalar = cmp::max(greatest_common_divisor(x_unscaled, y_unscaled), 1);
        let x = x_unscaled / scalar;
        let y = y_unscaled / scalar;

        Self {
            vector: ChessVector::new(x, y),
            scalar: scalar as u8,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChessVector {
    pub x: i8,
    pub y: i8,
}

impl ChessVector {
    // Factories.
    pub fn new(x: i8, y: i8) -> Self {
        Self { x: x, y: y }
    }

    /// Return the unit vector representing "forwards".
    /// This is different for black/white, since they play in opposite directions.
    pub fn forwards(colour: &chess_set::Colour) -> Self {
        let y = match colour {
            chess_set::Colour::White => 1,
            chess_set::Colour::Black => -1,
        };
        Self { x: 0, y: y }
    }

    pub fn right(colour: &chess_set::Colour) -> Self {
        let x = match colour {
            chess_set::Colour::White => 1,
            chess_set::Colour::Black => -1,
        };
        Self { x: x, y: 0 }
    }
}

/// Euclid's algorithm, extended to support negative numbers.
fn greatest_common_divisor(a: i8, b: i8) -> i8 {
    let (a_abs, b_abs) = (a.abs(), b.abs());
    if a_abs == b_abs {
        return a_abs;
    };

    let higher = cmp::max(a_abs, b_abs);
    let lower = cmp::min(a_abs, b_abs);

    if lower == 0 {
        return higher;
    };

    let difference = higher - lower;

    if difference == lower {
        lower
    } else {
        greatest_common_divisor(difference, lower)
    }
}

// Trait implementations.
impl ops::Mul<i8> for ChessVector {
    type Output = ChessVector;

    fn mul(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Add<Self> for ChessVector {
    type Output = Self;

    fn add(self, rhs: ChessVector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Self> for ChessVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl fmt::Display for ChessVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod translation_new_tests {
        use super::super::chess_set::{File, Rank, Square};
        use super::super::{ChessVector, Translation};
        use crate::testing::factories;
        use rstest::rstest;

        #[rstest]
        #[case::forwards(
            Square::new(Rank::One, File::A),
            Square::new(Rank::Two, File::A),
            ChessVector::new(0, 1)
        )]
        #[case::forwards_and_right(
            Square::new(Rank::Three, File::C),
            Square::new(Rank::Four, File::D),
            ChessVector::new(1, 1)
        )]
        #[case::right(
            Square::new(Rank::Seven, File::F),
            Square::new(Rank::Seven, File::G),
            ChessVector::new(1, 0)
        )]
        #[case::backwards_and_right(
            Square::new(Rank::Two, File::B),
            Square::new(Rank::One, File::C),
            ChessVector::new(1, -1)
        )]
        #[case::backwards(
            Square::new(Rank::Five, File::E),
            Square::new(Rank::Four, File::E),
            ChessVector::new(0, -1)
        )]
        #[case::backwards_and_left(
            Square::new(Rank::Eight, File::H),
            Square::new(Rank::Seven, File::G),
            ChessVector::new(-1, -1)
        )]
        #[case::left(
            Square::new(Rank::Four, File::D),
            Square::new(Rank::Four, File::C),
            ChessVector::new(-1, 0)
        )]
        #[case::forward_and_left(
            Square::new(Rank::Six, File::H),
            Square::new(Rank::Seven, File::G),
            ChessVector::new(-1, 1)
        )]
        fn single_square_moves(
            #[case] from_square: Square,
            #[case] to_square: Square,
            #[case] expected_vector: ChessVector,
        ) {
            let translation = Translation::from_move(&from_square, &to_square);

            assert_eq!(translation.vector, expected_vector);
            assert_eq!(translation.scalar, 1);
        }

        #[rstest]
        #[case::forwards(
            Square::new(Rank::One, File::A),
            Square::new(Rank::Three, File::A),
            ChessVector::new(0, 1),
            2
        )]
        #[case::forwards_and_right(
            Square::new(Rank::Three, File::C),
            Square::new(Rank::Six, File::F),
            ChessVector::new(1, 1),
            3
        )]
        #[case::right(
            Square::new(Rank::Seven, File::D),
            Square::new(Rank::Seven, File::H),
            ChessVector::new(1, 0),
            4
        )]
        #[case::backwards_and_right(
            Square::new(Rank::Six, File::A),
            Square::new(Rank::One, File::F),
            ChessVector::new(1, -1),
            5,
        )]
        #[case::backwards(
            Square::new(Rank::Seven, File::G),
            Square::new(Rank::Two, File::G),
            ChessVector::new(0, -1),
            5,
        )]
        #[case::backwards_and_left(
            Square::new(Rank::Eight, File::G),
            Square::new(Rank::Two, File::A),
            ChessVector::new(-1, -1),
            6,
        )]
        #[case::left(
            Square::new(Rank::Three, File::H),
            Square::new(Rank::Three, File::A),
            ChessVector::new(-1, 0),
            7,
        )]
        #[case::forward_and_left(
            Square::new(Rank::Four, File::F),
            Square::new(Rank::Eight, File::B),
            ChessVector::new(-1, 1),
            4,
        )]
        fn multiple_square_moves_in_a_straight_line(
            #[case] from_square: Square,
            #[case] to_square: Square,
            #[case] expected_vector: ChessVector,
            #[case] expected_scalar: u8,
        ) {
            let translation = Translation::from_move(&from_square, &to_square);

            assert_eq!(translation.vector, expected_vector);
            assert_eq!(translation.scalar, expected_scalar);
        }

        #[rstest]
        #[case::forwards_two_right_one(
            Square::new(Rank::One, File::A),
            Square::new(Rank::Three, File::B),
            ChessVector::new(1, 2)
        )]
        #[case::forwards_one_right_two(
            Square::new(Rank::Three, File::C),
            Square::new(Rank::Four, File::E),
            ChessVector::new(2, 1)
        )]
        #[case::backwards_one_right_two(
            Square::new(Rank::Three, File::C),
            Square::new(Rank::Two, File::E),
            ChessVector::new(2, -1),
        )]
        #[case::backwards_two_right_one(
            Square::new(Rank::Six, File::A),
            Square::new(Rank::Four, File::B),
            ChessVector::new(1, -2),
        )]
        #[case::backwards_two_left_one(
            Square::new(Rank::Seven, File::G),
            Square::new(Rank::Five, File::F),
            ChessVector::new(-1, -2),
        )]
        #[case::backwards_one_left_two(
            Square::new(Rank::Eight, File::G),
            Square::new(Rank::Seven, File::E),
            ChessVector::new(-2, -1),
        )]
        #[case::forwards_one_left_two(
            Square::new(Rank::Three, File::H),
            Square::new(Rank::Four, File::F),
            ChessVector::new(-2, 1),
        )]
        #[case::forwards_two_left_one(
            Square::new(Rank::Four, File::F),
            Square::new(Rank::Six, File::E),
            ChessVector::new(-1, 2),
        )]
        fn multiple_square_moves_in_a_wonky_line_scalar_factor_one(
            #[case] from_square: Square,
            #[case] to_square: Square,
            #[case] expected_vector: ChessVector,
        ) {
            let translation = Translation::from_move(&from_square, &to_square);

            assert_eq!(translation.vector, expected_vector);
            assert_eq!(translation.scalar, 1);
        }

        #[test]
        fn multiple_square_moves_in_a_wonky_line_with_scalar_factor() {
            let from_square = Square::new(Rank::Three, File::E);
            let to_square = Square::new(Rank::Seven, File::C);

            let expected_vector = ChessVector::new(-1, 2);
            let expected_scalar_factor = 2;

            let translation = Translation::from_move(&from_square, &to_square);

            assert_eq!(translation.vector, expected_vector);
            assert_eq!(translation.scalar, expected_scalar_factor);
        }

        #[test]
        fn move_to_same_square_does_not_panic() {
            let square = factories::some_square();
            let translation = Translation::from_move(&square, &square);

            assert_eq!(translation.vector, ChessVector::new(0, 0),);
            assert_eq!(translation.scalar, 1);
        }
    }

    #[cfg(test)]
    mod greatest_common_divisor_tests {
        use super::super::*;
        use rstest::rstest;

        #[rstest]
        #[case::both_zero(0, 0, 0)]
        #[case::zero_first(7, 0, 7)]
        #[case::zero_second(0, 5, 5)]
        #[case::prime_numbers(13, 11, 1)]
        #[case::same_number(31, 31, 31)]
        #[case::multiples_of_three(9, 12, 3)]
        #[case::order_does_not_matter(12, 9, 3)]
        #[case::multiples_of_two(6, 8, 2)]
        #[case::negative_number_first(-6, 8, 2)]
        #[case::negative_number_second(6, -8, 2)]
        #[case::both_numbers_negative(-6, -8, 2)]
        fn correctly_calculates_greatest_common_divisor(
            #[case] a: i8,
            #[case] b: i8,
            #[case] gcd: i8,
        ) {
            assert_eq!(greatest_common_divisor(a, b), gcd);
        }
    }
}
