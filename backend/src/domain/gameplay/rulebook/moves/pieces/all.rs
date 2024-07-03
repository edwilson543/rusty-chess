use super::super::ordinary_move;
use super::{bishop, king, knight, pawn, queen, rook};
use crate::domain::gameplay::chess_set;
use std::vec;

pub fn get_rules_for_piece(
    piece_type: &chess_set::PieceType,
) -> vec::IntoIter<Box<dyn ordinary_move::OrdinaryMoveRule>> {
    match piece_type {
        chess_set::PieceType::Pawn => pawn::get_pawn_move_rules(),
        chess_set::PieceType::Knight => knight::get_knight_move_rules(),
        chess_set::PieceType::Bishop => bishop::get_bishop_move_rules(),
        chess_set::PieceType::Rook => rook::get_rook_move_rules(),
        chess_set::PieceType::Queen => queen::get_queen_move_rules(),
        chess_set::PieceType::King => king::get_king_move_rules(),
    }
}
