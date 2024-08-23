use crate::repository;

/// Get the concrete game repository to use.
pub fn get_game_repo() -> Box<dyn repository::GameRepository> {
    let repo = repository::DieselGameRepository::new();
    Box::new(repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_game_repo() {
        let _ = get_game_repo();
    }
}
