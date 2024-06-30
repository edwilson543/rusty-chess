use super::super::rule;
use super::pawn;
use crate::domain::gameplay::chess_set;
use std::vec;

pub fn get_rules_for_piece(
    piece_type: &chess_set::PieceType,
) -> vec::IntoIter<Box<dyn rule::Rule>> {
    match piece_type {
        chess_set::PieceType::Pawn => pawn::get_pawn_move_rules(),
        _ => panic!("todo"),
    }
}
