use chess::repository::{DieselGameRepository, GameRepository};

#[cfg(test)]
mod get_and_create_tests {
    use super::*;
    use chess::domain::gameplay::game;
    use chess::repository::GameRepository;

    #[test]
    fn gets_game_when_exists() {
        let mut repo = DieselGameRepository::new();
        let created_game = repo.create();

        let maybe_game = repo.get(*created_game.get_id());
        let got_game = maybe_game.unwrap();

        let expected_new_game = game::Game::new(*created_game.get_id());
        assert_eq!(got_game, expected_new_game);
    }

    #[test]
    fn gets_none_when_game_does_not_exist() {
        let mut repo = DieselGameRepository::new();

        let maybe_game = repo.get(123);

        assert_eq!(maybe_game, None);
    }
}
