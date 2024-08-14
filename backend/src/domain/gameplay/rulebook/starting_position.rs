use crate::domain::gameplay::chess_set;
use std::collections::BTreeMap;

pub fn get_official_starting_position() -> BTreeMap<chess_set::Square, chess_set::Piece> {
    let mut starting_position = BTreeMap::new();

    let home_rank = get_home_rank();
    for (index, file) in chess_set::File::iter().enumerate() {
        // Add the pieces.
        let piece_type = home_rank[index];

        let square = chess_set::Square::new(chess_set::Rank::One, file);
        let white_piece = chess_set::Piece::new(chess_set::Colour::White, piece_type);
        starting_position.insert(square, white_piece);

        let square = chess_set::Square::new(chess_set::Rank::Eight, file);
        let black_piece = chess_set::Piece::new(chess_set::Colour::Black, piece_type);
        starting_position.insert(square, black_piece);

        // Add the pawns.
        let square = chess_set::Square::new(chess_set::Rank::Two, file);
        let pawn = chess_set::Piece::new(chess_set::Colour::White, chess_set::PieceType::Pawn);
        starting_position.insert(square, pawn);

        let square = chess_set::Square::new(chess_set::Rank::Seven, file);
        let pawn = chess_set::Piece::new(chess_set::Colour::Black, chess_set::PieceType::Pawn);
        starting_position.insert(square, pawn);
    }

    starting_position
}

fn get_home_rank() -> Vec<chess_set::PieceType> {
    return vec![
        chess_set::PieceType::Rook,
        chess_set::PieceType::Knight,
        chess_set::PieceType::Bishop,
        chess_set::PieceType::Queen,
        chess_set::PieceType::King,
        chess_set::PieceType::Bishop,
        chess_set::PieceType::Knight,
        chess_set::PieceType::Rook,
    ];
}

#[cfg(test)]
mod tests {
    use super::{
        chess_set::{Colour, File, Piece, PieceType, Rank, Square},
        get_official_starting_position,
    };

    #[test]
    fn creates_btree_map_representing_starting_position() {
        let starting_position = get_official_starting_position();

        let spot_checks = [
            (
                Square::new(Rank::One, File::A),
                Piece::new(Colour::White, PieceType::Rook),
            ),
            (
                Square::new(Rank::Two, File::E),
                Piece::new(Colour::White, PieceType::Pawn),
            ),
            (
                Square::new(Rank::Eight, File::D),
                Piece::new(Colour::Black, PieceType::Queen),
            ),
        ];

        for check in spot_checks.into_iter() {
            assert_eq!(starting_position.get(&check.0).unwrap(), &check.1);
        }
    }
}
