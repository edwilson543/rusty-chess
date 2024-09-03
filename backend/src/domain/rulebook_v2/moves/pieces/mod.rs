mod all_rules;
mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub use all_rules::{apply_move_if_allowed, is_move_allowed, MoveIsNotAllowed};
