use super::schema;
use diesel::prelude::*;

use crate::domain::gameplay::{chess_set, game};

#[derive(Selectable, Insertable)]
#[diesel(table_name = schema::game)]
pub struct Game {
    pub id: i32,
    pub status: i16,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::chessboard_square)]
pub struct ChessboardSquare {
    pub id: i32,
    pub game_id: i32,
    pub rank: i16,
    pub file: i16,
    pub chessboard_history_index: i16,
    pub piece_colour: Option<i16>,
    pub piece_type: Option<i16>,
}

impl Game {
    pub fn to_domain(&self) -> game::Game {
        game::Game::new(1) // TODO.
    }
}

impl ChessboardSquare {
    // Domain factories.

    fn to_domain_square(&self) -> chess_set::Square {
        let rank = chess_set::Rank::from_index(self.rank as i8);
        let file = chess_set::File::from_index(self.file as i8);
        chess_set::Square::new(rank, file)
    }

    fn to_domain_piece(&self) -> Option<chess_set::Piece> {
        let Some(piece_type) = &self.piece_type else {
            return None;
        };
        let Some(piece_colour) = &self.piece_colour else {
            return None;
        };

        let colour = chess_set::Colour::from_index(*piece_colour);
        let piece_type = chess_set::PieceType::from_index(*piece_type);

        Some(chess_set::Piece::new(colour, piece_type))
    }
}

// Db specific serializers & deserializers.

impl chess_set::Colour {
    fn to_index(&self) -> i16 {
        match &self {
            chess_set::Colour::White => 0,
            chess_set::Colour::Black => 1,
        }
    }

    fn from_index(index: i16) -> chess_set::Colour {
        match index {
            0 => chess_set::Colour::White,
            1 => chess_set::Colour::Black,
            _ => panic!("Invalid colour index!")
        }
    }
}

impl chess_set::PieceType {
    fn to_index(&self) -> i16 {
        match &self {
            chess_set::PieceType::Pawn => 0,
            chess_set::PieceType::Knight => 1,
            chess_set::PieceType::Bishop => 2,
            chess_set::PieceType::Rook => 3,
            chess_set::PieceType::Queen => 4,
            chess_set::PieceType::King => 5,
        }
    }

    fn from_index(index: i16) -> chess_set::PieceType {
        match index {
            0 => chess_set::PieceType::Pawn,
            1 => chess_set::PieceType::Knight,
            2 => chess_set::PieceType::Bishop,
            3 => chess_set::PieceType::Rook,
            4 => chess_set::PieceType::Queen,
            5 => chess_set::PieceType::King,
            _ => panic!("Invalid piece index!")
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod chessboard_square_domain_factory_tests {
        use super::super::ChessboardSquare;
        use crate::domain::gameplay::chess_set::{Rank, File, PieceType, Colour};

        #[test]
        fn chessboard_square_is_deserialized_to_a_square() {
            let db_square = ChessboardSquare {
                id: 1,
                game_id: 2,
                rank: 3,
                file: 7,
                chessboard_history_index: 4,
                piece_colour: None,
                piece_type: None,
            };

            let domain_square = db_square.to_domain_square();

            assert_eq!(domain_square.get_rank(), &Rank::Three);
            assert_eq!(domain_square.get_file(), &File::G);
        }

        #[test]
        fn chessboard_square_is_deserialized_to_a_piece_when_occupied() {
            let db_square = ChessboardSquare {
                id: 1,
                game_id: 2,
                rank: 3,
                file: 7,
                chessboard_history_index: 4,
                piece_colour: Some(1),
                piece_type: Some(3),
            };

            let domain_piece = db_square.to_domain_piece();

            domain_piece.unwrap_or_else(|| panic!("Square should be occupied!"));
            assert_eq!(domain_piece.unwrap().get_colour(), &Colour::Black);
            assert_eq!(domain_piece.unwrap().get_piece_type(), &PieceType::Rook);
        }

        #[test]
        fn chessboard_square_is_deserialized_to_null_when_unoccupied() {
            let db_square = ChessboardSquare {
                id: 1,
                game_id: 2,
                rank: 3,
                file: 7,
                chessboard_history_index: 4,
                piece_colour: None,
                piece_type: None,
            };

            let domain_piece = db_square.to_domain_piece();

            assert_eq!(domain_piece, None);
        }
    }
}