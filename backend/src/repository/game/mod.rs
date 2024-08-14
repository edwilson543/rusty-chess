mod diesel;
mod fake;
mod repo;

pub use diesel::DieselGameRepository;
pub use fake::FakeGameRepository;
pub use repo::GameRepository;
