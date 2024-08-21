use crate::repository;
use crate::services::unit_of_work;

/// Get the concrete uow implementation to use.
pub fn get_unit_of_work() -> Box<dyn unit_of_work::UnitOfWork> {
    let uow = unit_of_work::DieselUnitOfWork::new();
    Box::new(uow)
}

/// Get the concrete game repository to use.
pub fn get_game_repo() -> Box<dyn repository::GameRepository> {
    let repo = repository::DieselGameRepository::new();
    Box::new(repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_unit_of_work() {
        let uow = get_unit_of_work();

        uow.get_game_repo();
    }

    #[test]
    fn can_get_game_repo() {
        let _ = get_game_repo();
    }
}
