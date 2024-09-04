use super::engine;
use crate::domain::{game, rulebook};

pub struct MonteCarloTreeSearch;

impl engine::ChessEngine for MonteCarloTreeSearch {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook::Move, engine::SuggestNextMoveError> {
        todo!()
    }
}
