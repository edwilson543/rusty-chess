use super::engine;
use crate::domain::{game, rulebook_v2};

pub struct Minimax;

impl engine::ChessEngine for Minimax {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook_v2::Move, engine::SuggestNextMoveError> {
        todo!()
    }
}
