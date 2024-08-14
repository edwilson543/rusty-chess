mod diesel;
mod fake;
mod repo;

pub use fake::FakeGameRepository;
pub use repo::GameRepository;
pub use diesel::DieselGameRepository;
