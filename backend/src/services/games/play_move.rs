use crate::domain::chess_set;
use crate::domain::game;
use crate::repository;

use thiserror;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PlayMoveError {
    #[error("Game {0} does not exist")]
    GameDoesNotExist(i32),

    #[error("{0}")]
    InvalidMove(game::GameError),
}

pub fn play_move(
    mut game_repo: Box<dyn repository::GameRepository>,
    game_id: &i32,
    player: &chess_set::Colour,
    from_square: &chess_set::Square,
    to_square: &chess_set::Square,
) -> Result<game::Game, PlayMoveError> {
    let Some(mut game) = game_repo.get(&game_id) else {
        return Err(PlayMoveError::GameDoesNotExist(game_id.clone()));
    };

    match game.play_unvalidated_move(&player, &from_square, &to_square) {
        Err(err) => return Err(PlayMoveError::InvalidMove(err)),
        Ok(_) => {}
    }

    game_repo.update(&game);
    Ok(game)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chess_set::{Colour, File, Rank, Square};
    use crate::repository::GameRepository;

    #[test]
    fn can_play_valid_move() {
        let mut game_repo = repository::DieselGameRepository::new();
        let game = game_repo.create();

        let from_square = Square::new(Rank::Two, File::D);
        let to_square = Square::new(Rank::Four, File::D);

        let result = play_move(
            Box::new(game_repo),
            game.get_id(),
            &Colour::White,
            &from_square,
            &to_square,
        );

        let updated_game = result.unwrap();

        assert_eq!(updated_game.get_status(), &game::GameStatus::ToPlayBlack);
        assert_eq!(updated_game.get_chessboard_history().len(), 2);
        assert_eq!(updated_game.get_piece_at_square(&from_square), None);
        assert_ne!(updated_game.get_piece_at_square(&to_square), None);
    }

    #[test]
    fn errors_when_game_does_not_exist() {
        let game_repo = repository::DieselGameRepository::new();
        let invalid_id = 37;

        let result = play_move(
            Box::new(game_repo),
            &invalid_id,
            &Colour::White,
            &Square::new(Rank::Two, File::D),
            &Square::new(Rank::Four, File::D),
        );

        assert_eq!(result, Err(PlayMoveError::GameDoesNotExist(invalid_id)))
    }

    #[test]
    fn errors_when_move_is_out_of_turn() {
        let mut game_repo = repository::DieselGameRepository::new();
        let game = game_repo.create();

        let result = play_move(
            Box::new(game_repo),
            game.get_id(),
            &Colour::Black,
            &Square::new(Rank::Two, File::D),
            &Square::new(Rank::Four, File::D),
        );

        let game_error = game::GameError::PlayIsOutOfTurn(Colour::Black);
        assert_eq!(result, Err(PlayMoveError::InvalidMove(game_error)))
    }
}
