mod all;
mod bishop;
mod common;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
mod rule;

pub use all::get_rules_for_piece;
pub use rule::OrdinaryMoveRule;
