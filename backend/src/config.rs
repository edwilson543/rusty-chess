use crate::services::unit_of_work;

/// Get the concrete uow implementation to use.
pub fn get_unit_of_work() -> Box<dyn unit_of_work::UnitOfWork> {
    let uow = unit_of_work::FakeUnitOfWork::new();
    Box::new(uow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_unit_of_work() {
        let uow = get_unit_of_work();

        uow.get_game_repo();
    }
}
