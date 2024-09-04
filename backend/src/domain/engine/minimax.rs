use super::engine;
use crate::domain::{game, rulebook};

pub struct Minimax;

impl engine::ChessEngine for Minimax {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook::Move, engine::SuggestNextMoveError> {
        todo!()
    }
}
