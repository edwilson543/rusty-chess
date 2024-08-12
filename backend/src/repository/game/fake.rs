use super::repo;
use crate::domain::gameplay::game;
use std::collections::BTreeMap;

pub struct FakeGameRepository {
    games: BTreeMap<i32, game::Game>,
}

impl repo::GameRepository for FakeGameRepository {
    fn get(&self, id: i32) -> Option<game::Game> {
        let Some(game) = self.games.get(&id) else {
            return None;
        };
        Some(game.clone())
    }

    fn create(&mut self) -> game::Game {
        let id = self.get_next_id();
        let game = game::Game::new(id);
        self.games.insert(id, game.clone());
        game
    }

    fn update(&mut self, game: &game::Game) {
        self.games.insert(game.get_id().clone(), game.clone());
    }
}

impl FakeGameRepository {
    pub fn new() -> Self {
        Self {
            games: BTreeMap::new(),
        }
    }

    fn get_next_id(&self) -> i32 {
        let Some((max_id, _)) = self.games.iter().next_back() else {
            return 1;
        };
        max_id + 1
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod get_tests {
        use super::super::repo::GameRepository;
        use super::super::FakeGameRepository;
        use crate::domain::gameplay::game;

        #[test]
        fn gets_game_when_exists() {
            let mut repo = FakeGameRepository::new();
            let game = game::Game::new(3);
            repo.games.insert(3, game.clone());

            let result = repo.get(3);

            assert_eq!(result, Some(game))
        }

        #[test]
        fn gets_none_when_game_does_not_exist() {
            let repo = FakeGameRepository::new();

            let result = repo.get(7);

            assert_eq!(result, None)
        }
    }

    #[cfg(test)]
    mod create_tests {
        use super::super::repo::GameRepository;
        use super::super::FakeGameRepository;
        use crate::domain::gameplay::game;

        #[test]
        fn creates_first_game_with_id_one() {
            let mut repo = FakeGameRepository::new();

            let result = repo.create();

            assert_eq!(result.get_id(), &1);
            assert_eq!(result.get_chessboard_history().len(), 1);
        }

        #[test]
        fn creates_subsequent_game_with_id_greater_than_one() {
            let mut repo = FakeGameRepository::new();
            let game = game::Game::new(9);
            repo.games.insert(9, game.clone());

            let result = repo.create();

            assert_eq!(result.get_id(), &10);
            assert_eq!(result.get_chessboard_history().len(), 1);
        }
    }

    #[cfg(test)]
    mod update_tests {
        use super::super::repo::GameRepository;
        use super::super::FakeGameRepository;
        use crate::domain::gameplay::chess_set::{Colour, File, Rank, Square};
        use crate::domain::gameplay::game;

        #[test]
        fn updates_game_to_new_instance() {
            let mut repo = FakeGameRepository::new();
            let mut game = game::Game::new(2);
            repo.games.insert(2, game.clone());

            let from_square = Square::new(Rank::Two, File::A);
            let to_square = Square::new(Rank::Three, File::A);
            game.play_ordinary_move(&Colour::White, &from_square, &to_square)
                .unwrap();

            repo.update(&game);

            let updated_game = repo.games.get(&2).unwrap();
            assert_eq!(updated_game, &game);
            assert_eq!(updated_game.get_chessboard_history().len(), 2);
        }
    }
}
