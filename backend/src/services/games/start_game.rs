use crate::domain::gameplay;
use crate::domain::gameplay::chess_set;
use crate::services::unit_of_work;

pub fn start_game(uow: Box<dyn unit_of_work::UnitOfWork>) -> gameplay::Game {
    let mut game_repo = uow.get_game_repo();
    game_repo.create()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_start_game() {
        let uow = unit_of_work::FakeUnitOfWork::new();

        let game = start_game(Box::new(uow));

        assert_eq!(
            game.get_status(),
            &gameplay::GameStatus::ToPlay(chess_set::Colour::White)
        );
        assert_eq!(game.get_chessboard_history().len(), 1);
    }
}
