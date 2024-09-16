use crate::domain::engine;
use crate::repository;

/// Get the concrete game repository to use.
pub fn get_game_repo() -> Box<dyn repository::GameRepository> {
    let repo = repository::DieselGameRepository::new();
    Box::new(repo)
}

#[derive(serde::Deserialize)]
pub enum ChessEngineImplementation {
    Random,
    Minimax,
    MCTS,
}

/// Get the concrete chess engine to use.
pub fn get_chess_engine(
    implementation: &ChessEngineImplementation,
) -> Box<dyn engine::ChessEngine> {
    match implementation {
        ChessEngineImplementation::Random => get_random_chess_engine(),
        ChessEngineImplementation::Minimax => get_minimax_chess_engine(),
        ChessEngineImplementation::MCTS => get_mcts_chess_engine(),
    }
}

fn get_random_chess_engine() -> Box<dyn engine::ChessEngine> {
    let chess_engine = engine::Random::new();
    Box::new(chess_engine)
}

fn get_minimax_chess_engine() -> Box<dyn engine::ChessEngine> {
    let evaluator = get_chessboard_evaluator();
    let max_search_depth = 3;
    let chess_engine = engine::Minimax::new(evaluator, max_search_depth);
    Box::new(chess_engine)
}

fn get_mcts_chess_engine() -> Box<dyn engine::ChessEngine> {
    let chess_engine = engine::MonteCarloTreeSearch::new();
    Box::new(chess_engine)
}

fn get_chessboard_evaluator() -> Box<dyn engine::ChessboardEvaluator> {
    let evaluator = engine::PiecePlacementChessboardEvaluator;
    Box::new(evaluator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_game_repo() {
        let _ = get_game_repo();
    }
}
