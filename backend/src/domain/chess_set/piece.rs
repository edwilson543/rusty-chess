#[derive(Copy, Clone, Debug)]
pub enum Colour {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Rook,
    Queen,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub colour: Colour,
    pub piece_type: PieceType,
}
