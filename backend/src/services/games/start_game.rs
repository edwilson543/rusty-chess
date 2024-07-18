use crate::domain::gameplay::game;
use crate::services::unit_of_work;

pub fn start_game(uow: Box<dyn unit_of_work::UnitOfWork>) -> game::Game {
    let mut game_repo = uow.get_game_repo();
    game_repo.create()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gameplay::chess_set;

    #[test]
    fn can_start_game() {
        let uow = unit_of_work::FakeUnitOfWork::new();

        let game = start_game(Box::new(uow));

        assert_eq!(
            game.get_status(),
            &game::GameStatus::ToPlay(chess_set::Colour::White)
        );
        assert_eq!(game.get_chessboard_history().len(), 1);
    }
}
