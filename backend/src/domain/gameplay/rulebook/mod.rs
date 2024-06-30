mod castle;
mod moves;
mod starting_position;

pub use moves::{validate_move, MoveValidationError};
pub use starting_position::get_official_starting_position;
