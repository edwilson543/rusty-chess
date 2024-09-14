use crate::domain::engine;
use crate::repository;

/// Get the concrete game repository to use.
pub fn get_game_repo() -> Box<dyn repository::GameRepository> {
    let repo = repository::DieselGameRepository::new();
    Box::new(repo)
}

/// Get the concrete chess engine to use.
pub fn get_chess_engine() -> Box<dyn engine::ChessEngine> {
    get_minimax_chess_engine()
}

pub fn get_random_chess_engine() -> Box<dyn engine::ChessEngine> {
    let chess_engine = engine::Random::new();
    Box::new(chess_engine)
}

pub fn get_minimax_chess_engine() -> Box<dyn engine::ChessEngine> {
    let evaluator = get_chessboard_evaluator();
    let max_search_depth = 3;
    let chess_engine = engine::Minimax::new(evaluator, max_search_depth);
    Box::new(chess_engine)
}

pub fn get_chessboard_evaluator() -> Box<dyn engine::ChessboardEvaluator> {
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
