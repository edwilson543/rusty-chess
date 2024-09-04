use super::engine;
use crate::domain::{game, rulebook_v2};

pub struct MonteCarloTreeSearch;

impl engine::ChessEngine for MonteCarloTreeSearch {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook_v2::Move, engine::SuggestNextMoveError> {
        todo!()
    }
}
