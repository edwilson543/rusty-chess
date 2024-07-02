use super::super::move_rule;
use super::{knight, pawn};
use crate::domain::gameplay::chess_set;
use std::vec;

pub fn get_rules_for_piece(
    piece_type: &chess_set::PieceType,
) -> vec::IntoIter<Box<dyn move_rule::MoveRule>> {
    match piece_type {
        chess_set::PieceType::Pawn => pawn::get_pawn_move_rules(),
        chess_set::PieceType::Knight => knight::get_knight_move_rules(),
        _ => panic!("todo"),
    }
}
