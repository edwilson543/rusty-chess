mod castle;
mod en_passant;
mod moves;
mod starting_position;

pub use en_passant::{validate_en_passant, EnPassant, EnPassantValidationError};
pub use moves::{validate_move, MoveValidationError, OrdinaryMove};
pub use starting_position::get_official_starting_position;
