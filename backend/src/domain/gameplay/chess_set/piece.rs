use serde;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Colour {
    Black,
    White,
}

impl Colour {
    pub fn swap(&self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn, // For convenience, pawns are modelled as `pieces` within the chess set.
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
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

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}

// Trait implementations.

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colour::White => write!(f, "W"),
            Colour::Black => write!(f, "B"),
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "P"),
            PieceType::Knight => write!(f, "N"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::King => write!(f, "K"),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.colour, self.piece_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::gameplay::chess_set;

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

    #[test]
    fn serializes_colour_to_json_then_deserializes_back_to_colour() {
        let colour = chess_set::Colour::Black;

        let serialized = serde_json::to_string(&colour).unwrap();

        assert_eq!(serialized, "\"Black\"");

        let deserialized: chess_set::Colour = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, colour);
    }
}
