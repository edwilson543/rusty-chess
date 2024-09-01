use super::engine::ChessEngine;
use crate::domain::engine::engine;
use crate::domain::game;
use crate::domain::rulebook;

pub struct MonteCarloTreeSearch;

impl ChessEngine for MonteCarloTreeSearch {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<Box<dyn rulebook::Move>, engine::SuggestNextMoveError> {
        todo!()
    }
}
