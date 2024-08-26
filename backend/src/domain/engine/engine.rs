use crate::domain::{game, rulebook};

/// A type that is capable of suggesting chess moves.
///
/// Example implementations include Minimax and MCTS.
pub trait ChessEngine {
    fn suggest_next_move(game: &game::Game) -> Box<dyn rulebook::Move>;
}
