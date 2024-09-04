use super::engine;
use crate::domain::game;
use crate::domain::rulebook_v2;

use rand::{thread_rng, Rng};

pub struct Random;

impl engine::ChessEngine for Random {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook_v2::Move, engine::SuggestNextMoveError> {
        let Some(to_play_colour) = game.get_status().to_play_colour() else {
            return Err(engine::SuggestNextMoveError::GameHasAlreadyEnded);
        };

        let mut legal_moves =
            rulebook_v2::get_legal_moves(to_play_colour, game.get_chessboard_history());

        let mut rng = thread_rng();
        let selected_move_index = rng.gen_range(0..legal_moves.len());
        let suggested_move = legal_moves.remove(selected_move_index);

        Ok(suggested_move)
    }
}

impl Random {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::engine::ChessEngine;
    use crate::domain::game;

    #[test]
    fn suggests_opening_move() {
        let game = game::Game::new(1);
        let engine = Random::new();

        let suggested_move = engine.generate_next_move(&game).unwrap();

        match suggested_move.validate(game.get_chessboard_history()) {
            Ok(_) => {}
            Err(_) => panic!("Random move engine generated an illegal move!"),
        }
    }

    #[test]
    fn returns_error_when_game_has_already_ended() {
        let game = game::Game::reincarnate(1, game::GameStatus::Drawn, vec![]);
        let engine = Random::new();

        let suggested_move = engine.generate_next_move(&game);

        let expected_error = engine::SuggestNextMoveError::GameHasAlreadyEnded;
        match suggested_move {
            Err(err) => assert_eq!(err, expected_error),
            Ok(_) => panic!("Not okay!"),
        }
    }
}
