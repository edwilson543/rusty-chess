use super::schema;
use diesel::prelude::*;

use crate::domain::{chess_set, game};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::game)]
pub struct Game {
    pub id: i32,
    pub status: i16,
}

#[derive(Insertable)]
#[diesel(table_name = schema::game)]
struct NewGame {
    status: i16,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = schema::occupied_chessboard_square)]
pub struct OccupiedChessboardSquare {
    pub id: i32,
    pub game_id: i32,
    pub chessboard_history_index: i16,
    pub rank: i16,
    pub file: i16,
    pub piece_colour: i16,
    pub piece_type: i16,
}

#[derive(Insertable)]
#[diesel(table_name = schema::occupied_chessboard_square)]
struct NewChessboardSquare {
    game_id: i32,
    chessboard_history_index: i16,
    rank: i16,
    file: i16,
    piece_colour: i16,
    piece_type: i16,
}

impl Game {
    // SQL.
    pub fn get(conn: &mut PgConnection, id: &i32) -> Option<Self> {
        use crate::data::schema::game::dsl;

        let game = dsl::game
            .find(id)
            .select(Game::as_select())
            .first(conn)
            .optional();

        match game {
            Ok(maybe_game) => maybe_game,
            Err(err) => panic!("{}", err),
        }
    }

    pub fn create(conn: &mut PgConnection, status: game::GameStatus) -> Self {
        use crate::data::schema::game;

        let new_game = NewGame {
            status: status.to_index(),
        };
        diesel::insert_into(game::table)
            .values(&new_game)
            .returning(Self::as_returning())
            .get_result(conn)
            .expect("Error saving new post")
    }

    pub fn update_status(conn: &mut PgConnection, updated_game: &game::Game) {
        use crate::data::schema::game::dsl::{game, status};

        let _ = diesel::update(game.find(updated_game.get_id()))
            .set(status.eq(updated_game.get_status().to_index()))
            .execute(conn);
    }

    // Domain factories.

    pub fn to_domain(&self, chessboard_history: Vec<chess_set::Chessboard>) -> game::Game {
        game::Game::reincarnate(
            self.id,
            game::GameStatus::from_index(self.status),
            chessboard_history,
        )
    }
}

impl OccupiedChessboardSquare {
    // SQL.

    pub fn bulk_create_for_latest_chessboard(conn: &mut PgConnection, game: &game::Game) {
        use crate::data::schema::occupied_chessboard_square;

        let chessboard_history_index = game.get_chessboard_history().len() - 1;
        let chessboard = game.current_chessboard().clone();

        let mut new_squares = vec![];
        for (square, maybe_piece) in chessboard.position.into_iter() {
            let Some(piece) = maybe_piece else { continue };

            let new_square = NewChessboardSquare {
                game_id: *game.get_id(),
                chessboard_history_index: chessboard_history_index as i16,
                rank: square.get_rank().index() as i16,
                file: square.get_file().index() as i16,
                piece_colour: piece.get_colour().to_index(),
                piece_type: piece.get_piece_type().to_index(),
            };
            new_squares.push(new_square);
        }

        let _ = diesel::insert_into(occupied_chessboard_square::table)
            .values(&new_squares)
            .execute(conn);
    }

    pub fn select_for_game(
        conn: &mut PgConnection,
        for_game_id: &i32,
    ) -> Vec<OccupiedChessboardSquare> {
        use crate::data::schema::occupied_chessboard_square::dsl::{
            chessboard_history_index, game_id, occupied_chessboard_square,
        };

        occupied_chessboard_square
            .filter(game_id.eq(for_game_id))
            .select(OccupiedChessboardSquare::as_select())
            .order(chessboard_history_index.asc())
            .load(conn)
            .expect("Error loading chessboard!")
    }

    // Domain factories.

    pub fn to_domain_square(&self) -> chess_set::Square {
        let rank = chess_set::Rank::from_index(self.rank as i8);
        let file = chess_set::File::from_index(self.file as i8);
        chess_set::Square::new(rank, file)
    }

    pub fn to_domain_piece(&self) -> chess_set::Piece {
        let colour = chess_set::Colour::from_index(self.piece_colour);
        let piece_type = chess_set::PieceType::from_index(self.piece_type);
        chess_set::Piece::new(colour, piece_type)
    }
}

// Db specific serializers & deserializers.

impl game::GameStatus {
    fn to_index(&self) -> i16 {
        match &self {
            game::GameStatus::ToPlayWhite => 0,
            game::GameStatus::ToPlayBlack => 1,
            game::GameStatus::WonByWhite => 2,
            game::GameStatus::WonByBlack => 3,
            game::GameStatus::Drawn => 4,
        }
    }

    fn from_index(index: i16) -> game::GameStatus {
        match index {
            0 => game::GameStatus::ToPlayWhite,
            1 => game::GameStatus::ToPlayBlack,
            2 => game::GameStatus::WonByWhite,
            3 => game::GameStatus::WonByBlack,
            4 => game::GameStatus::Drawn,
            _ => panic!("Invalid game status index!"),
        }
    }
}

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
            _ => panic!("Invalid colour index!"),
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
            _ => panic!("Invalid piece index!"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod occupied_chessboard_square_domain_factory_tests {
        use super::super::OccupiedChessboardSquare;
        use crate::domain::chess_set::{Colour, File, PieceType, Rank};

        #[test]
        fn occupied_chessboard_square_is_deserialized_to_a_square() {
            let db_square = OccupiedChessboardSquare {
                id: 1,
                game_id: 2,
                rank: 3,
                file: 7,
                chessboard_history_index: 4,
                piece_colour: 4,
                piece_type: 3,
            };

            let domain_square = db_square.to_domain_square();

            assert_eq!(domain_square.get_rank(), &Rank::Three);
            assert_eq!(domain_square.get_file(), &File::G);
        }

        #[test]
        fn occupied_chessboard_square_is_deserialized_to_a_piece() {
            let db_square = OccupiedChessboardSquare {
                id: 1,
                game_id: 2,
                rank: 3,
                file: 7,
                chessboard_history_index: 4,
                piece_colour: 1,
                piece_type: 3,
            };

            let domain_piece = db_square.to_domain_piece();

            assert_eq!(domain_piece.get_colour(), &Colour::Black);
            assert_eq!(domain_piece.get_piece_type(), &PieceType::Rook);
        }
    }

    #[cfg(test)]
    mod game_domain_factory_tests {
        use super::super::Game;
        use crate::domain::game;
        use crate::testing::factories;

        #[test]
        fn occupied_chessboard_square_is_deserialized_to_a_square() {
            let db_game = Game { id: 1, status: 1 };
            let chessboard = factories::chessboard();

            let domain_game = db_game.to_domain(vec![chessboard.clone()]);

            assert_eq!(domain_game.get_id(), &db_game.id);
            assert_eq!(domain_game.get_status(), &game::GameStatus::ToPlayBlack);
            assert_eq!(domain_game.get_chessboard_history(), &vec![chessboard]);
        }
    }
}
