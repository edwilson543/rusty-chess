use crate::domain::{game, rulebook};
use thiserror;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SuggestNextMoveError {
    #[error("The game has already ended.")]
    GameHasAlreadyEnded,
}

/// A type that is capable of suggesting chess moves.
///
/// Example implementations include Minimax and MCTS.
pub trait ChessEngine {
    fn generate_next_move(&self, game: &game::Game)
        -> Result<rulebook::Move, SuggestNextMoveError>;
}
