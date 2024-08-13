use crate::domain::gameplay::game;

pub trait GameRepository {
    fn get(&mut self, id: i32) -> Option<game::Game>;

    fn create(&mut self) -> game::Game;

    fn update(&mut self, game: &game::Game);
}
