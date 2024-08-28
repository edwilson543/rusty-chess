use crate::domain::engine;
use crate::repository;

/// Get the concrete game repository to use.
pub fn get_game_repo() -> Box<dyn repository::GameRepository> {
    let repo = repository::DieselGameRepository::new();
    Box::new(repo)
}

/// Get the concrete chess engine to use.
pub fn get_chess_engine() -> Box<dyn engine::ChessEngine> {
    let chess_engine = engine::Random::new();
    Box::new(chess_engine)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_game_repo() {
        let _ = get_game_repo();
    }
}
