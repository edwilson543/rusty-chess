use super::{bishop, pawn};
use crate::domain::chess_set;
use crate::domain::rulebook::moves_v2::chess_move;
use std::vec;

pub fn get_rules_for_piece(
    piece_type: &chess_set::PieceType,
) -> vec::IntoIter<Box<dyn chess_move::MoveRule>> {
    match piece_type {
        // chess_set::PieceType::Pawn => pawn::get_pawn_move_rules(),
        chess_set::PieceType::Bishop => bishop::get_bishop_move_rules(),
        _ => panic!("TODO!"),
    }
}
