use diesel::pg::PgConnection;
use std::collections::BTreeMap;

use super::repo;
use crate::data::{connection, models};
use crate::domain::gameplay::{chess_set, game};

pub struct DieselGameRepository {
    connection: PgConnection,
}

impl DieselGameRepository {
    pub fn new() -> Self {
        Self {
            connection: connection::establish_connection(),
        }
    }

    fn using_connection(connection: PgConnection) -> Self {
        Self {
            connection: connection,
        }
    }
}

impl repo::GameRepository for DieselGameRepository {
    fn get(&mut self, id: i32) -> Option<game::Game> {
        // TODO -> fetch actual history.
        let chessboard_squares =
            models::ChessboardSquare::select_for_game(&mut self.connection, &id);
        let chessboard_history =
            convert_chessboard_squares_to_chessboard_history(chessboard_squares);

        match models::Game::get(&mut self.connection, id) {
            Some(db_game) => Some(db_game.to_domain(chessboard_history)),
            None => None,
        }
    }

    fn create(&mut self) -> game::Game {
        let db_game = models::Game::create(&mut self.connection, game::GameStatus::ToPlayWhite);
        let game = game::Game::new(db_game.id);

        // Persist the initial chessboard.
        models::Game::update_chessboard(&mut self.connection, &game);

        game
    }

    fn update(&mut self, game: &game::Game) {
        todo!()
    }
}

fn convert_chessboard_squares_to_chessboard_history(
    squares: Vec<models::ChessboardSquare>,
) -> Vec<chess_set::Chessboard> {
    let mut chessboard_history: Vec<BTreeMap<chess_set::Square, chess_set::Piece>> = vec![];

    // Note that `squares` is already ordered by `chessboard_history_index`.
    for db_square in squares.iter() {
        match chessboard_history.get(db_square.chessboard_history_index as usize) {
            None => chessboard_history.push(BTreeMap::new()),
            Some(_) => {}
        };

        let square = db_square.to_domain_square();
        let Some(piece) = db_square.to_domain_piece() else {
            continue;
        };
        chessboard_history[db_square.chessboard_history_index as usize].insert(square, piece);
    }

    chessboard_history
        .iter()
        .map(|position| chess_set::Chessboard::new(position.clone()))
        .collect::<Vec<chess_set::Chessboard>>()
}
