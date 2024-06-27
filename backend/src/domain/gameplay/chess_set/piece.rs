use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Colour {
    Black,
    White,
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Colour {
    pub fn swap(&self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn, // For convenience, pawns are modelled as pieces within the chess set.
    Rook,
    Queen,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.colour, self.piece_type)
    }
}

impl Piece {
    // Factories.
    pub fn new(colour: Colour, piece_type: PieceType) -> Self {
        Self {
            colour: colour,
            piece_type: piece_type,
        }
    }

    // Queries.
    pub fn get_colour(&self) -> &Colour {
        &self.colour
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod colour_tests {
        use super::super::*;

        #[test]
        fn white_swaps_to_black() {
            assert_eq!(Colour::White.swap(), Colour::Black)
        }

        #[test]
        fn black_swaps_to_white() {
            assert_eq!(Colour::Black.swap(), Colour::White)
        }
    }
}
