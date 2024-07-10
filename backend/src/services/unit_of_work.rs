use crate::repository;

pub trait UnitOfWork {
    fn get_game_repo(&self) -> Box<dyn repository::GameRepository>;
}

pub struct FakeUnitOfWork;

impl UnitOfWork for FakeUnitOfWork {
    fn get_game_repo(&self) -> Box<dyn repository::GameRepository> {
        let repo = repository::FakeGameRepository::new();
        Box::new(repo)
    }
}

impl FakeUnitOfWork {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct_fake_unit_of_work() {
        let uow = FakeUnitOfWork::new();

        uow.get_game_repo();
    }
}
