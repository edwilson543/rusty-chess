use crate::domain::game;
use crate::repository;

pub fn start_game(mut game_repo: Box<dyn repository::GameRepository>) -> game::Game {
    game_repo.create()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_start_game() {
        let game_repo = repository::FakeGameRepository::new();

        let game = start_game(Box::new(game_repo));

        assert_eq!(game.get_status(), &game::GameStatus::ToPlayWhite);
        assert_eq!(game.get_chessboard_history().len(), 1);
    }
}
