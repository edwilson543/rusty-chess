use crate::domain::{engine, game};
use crate::repository;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum GenerateNextMoveError {
    #[error("Game {0} does not exist")]
    GameDoesNotExist(i32),

    #[error("The game has already ended.")]
    GameHasAlreadyEnded,

    #[error("{0}")]
    SuggestMoveError(engine::SuggestNextMoveError),
}

pub fn generate_and_play_next_move(
    mut game_repo: Box<dyn repository::GameRepository>,
    chess_engine: Box<dyn engine::ChessEngine>,
    game_id: i32,
) -> Result<game::Game, GenerateNextMoveError> {
    let Some(mut game) = game_repo.get(&game_id) else {
        return Err(GenerateNextMoveError::GameDoesNotExist(game_id));
    };

    let Some(_) = game.get_status().to_play_colour() else {
        return Err(GenerateNextMoveError::GameHasAlreadyEnded);
    };

    let move_to_play = match chess_engine.generate_next_move(&game) {
        Ok(move_to_play) => move_to_play,
        Err(err) => return Err(GenerateNextMoveError::SuggestMoveError(err)),
    };

    game.play_validated_move(move_to_play)
        .unwrap_or_else(|_| panic!("Engine generated an invalid move!"));

    game_repo.update(&game);

    Ok(game)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chess_set::{Colour, File, Rank, Square};
    use crate::repository::GameRepository;

    #[test]
    fn plays_move_for_white_generated_by_random_move_engine() {
        let mut game_repo = repository::FakeGameRepository::new();
        let game = game_repo.create();
        let engine = engine::Random::new();

        let result = generate_and_play_next_move(
            Box::new(game_repo),
            Box::new(engine),
            game.get_id().clone(),
        );

        let updated_game = result.unwrap();
        assert_eq!(updated_game.get_chessboard_history().len(), 2);
        assert_eq!(updated_game.get_status(), &game::GameStatus::ToPlayBlack);
    }

    #[test]
    fn plays_move_for_black_generated_by_random_move_engine() {
        let mut game_repo = repository::FakeGameRepository::new();
        let mut game = game_repo.create();
        let engine = engine::Random::new();

        // Play an opening move for white so that it's black's turn.
        let from_square = Square::new(Rank::Two, File::E);
        let to_square = Square::new(Rank::Four, File::E);
        game.play_unvalidated_move(&Colour::White, &from_square, &to_square)
            .unwrap();
        game_repo.update(&game);

        let result = generate_and_play_next_move(
            Box::new(game_repo),
            Box::new(engine),
            game.get_id().clone(),
        );

        let updated_game = result.unwrap();
        assert_eq!(updated_game.get_chessboard_history().len(), 3);
        assert_eq!(updated_game.get_status(), &game::GameStatus::ToPlayWhite);
    }

    #[test]
    fn returns_error_when_game_does_not_exist() {
        let game_repo = repository::FakeGameRepository::new();
        let engine = engine::Random::new();

        let result = generate_and_play_next_move(Box::new(game_repo), Box::new(engine), 123);

        let expected_error = GenerateNextMoveError::GameDoesNotExist(123);
        assert_eq!(result, Err(expected_error));
    }

    #[test]
    fn returns_error_when_game_has_already_ended() {
        let drawn_game = game::Game::reincarnate(1, game::GameStatus::Drawn, vec![]);
        let mut game_repo = repository::FakeGameRepository::new();
        game_repo.update(&drawn_game);

        let engine = engine::Random::new();

        let result = generate_and_play_next_move(
            Box::new(game_repo),
            Box::new(engine),
            drawn_game.get_id().clone(),
        );

        let expected_error = GenerateNextMoveError::GameHasAlreadyEnded;
        assert_eq!(result, Err(expected_error));
    }
}
