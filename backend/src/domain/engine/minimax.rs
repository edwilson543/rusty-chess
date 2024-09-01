use super::engine::ChessEngine;
use crate::domain::engine::engine;
use crate::domain::game;
use crate::domain::rulebook;

pub struct Minimax;

impl ChessEngine for Minimax {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<Box<dyn rulebook::Move>, engine::SuggestNextMoveError> {
        todo!()
    }
}
