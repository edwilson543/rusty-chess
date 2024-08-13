use super::repo;
use crate::data::{connection, models};
use crate::domain::gameplay::game;
pub struct DieselGameRepository;

impl repo::GameRepository for DieselGameRepository {
    fn get(&self, id: i32) -> Option<game::Game> {
        let connection = &mut connection::establish_connection();

        // TODO -> fetch actual history.
        let chessboard_history = vec![];

        match models::Game::get(connection, id) {
            Some(db_game) => Some(db_game.to_domain(chessboard_history)),
            None => None,
        }
    }

    fn create(&mut self) -> game::Game {
        let connection = &mut connection::establish_connection();

        let db_game = models::Game::create(connection, game::GameStatus::ToPlayWhite);
        let game = game::Game::new(db_game.id);

        // Persist the initial chessboard.
        models::Game::update_chessboard(connection, &game);

        game
    }

    fn update(&mut self, game: &game::Game) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod get_tests {
        use super::super::DieselGameRepository;
        use crate::repository::GameRepository;
        use crate::testing::factories;

        #[test]
        fn gets_game_when_exists() {
            let mut repo = DieselGameRepository;
            let created_game = repo.create();

            let maybe_game = repo.get(*created_game.get_id());
            let got_game = maybe_game.unwrap();

            assert_eq!(got_game.get_chessboard_history().len(), 1);
            assert_eq!(got_game.current_chessboard(), &factories::chessboard());
        }

        #[test]
        fn gets_none_when_game_does_not_exist() {
            let repo = DieselGameRepository;

            let maybe_game = repo.get(123);

            assert_eq!(maybe_game, None)
        }
    }
}