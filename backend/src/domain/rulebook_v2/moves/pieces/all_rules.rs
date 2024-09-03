use super::{bishop, king, knight, pawn, queen, rook};
use crate::domain::chess_set;
use crate::domain::rulebook_v2::moves::chess_move;
use std::vec;

use thiserror;

#[derive(thiserror::Error, Debug)]
pub struct MoveIsNotAllowed;


pub fn apply_move_if_allowed(
    chess_move: &chess_move::Move,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> Result<chess_set::Chessboard, MoveIsNotAllowed> {
    let rule = match get_rule_that_allows_move(chess_move, chessboard_history) {
        Some(rule) => rule,
        None => return Err(MoveIsNotAllowed),
    };

    let mut chessboard = chessboard_history.last().unwrap().clone();
    let move_outcome = rule.get_move_outcome(chess_move);
    chessboard.update_position(move_outcome);
    Ok(chessboard)
}

pub fn is_move_allowed(
    chess_move: &chess_move::Move,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> bool {
    match get_rule_that_allows_move(chess_move, chessboard_history) {
        Some(_) => true,
        None => false,
    }
}

pub fn get_rule_that_allows_move(
    chess_move: &chess_move::Move,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> Option<Box<dyn chess_move::MoveRule>> {
    let rules = get_move_rules_for_piece(chess_move.piece.get_piece_type());
    for rule in rules {
        if rule.allows_move(chess_move, chessboard_history) {
            return Some(rule);
        }
    }
    None
}

fn get_move_rules_for_piece(
    piece_type: &chess_set::PieceType,
) -> vec::IntoIter<Box<dyn chess_move::MoveRule>> {
    match piece_type {
        chess_set::PieceType::Pawn => pawn::get_pawn_move_rules(),
        chess_set::PieceType::Knight => knight::get_knight_move_rules(),
        chess_set::PieceType::Bishop => bishop::get_bishop_move_rules(),
        chess_set::PieceType::Rook => rook::get_rook_move_rules(),
        chess_set::PieceType::Queen => queen::get_queen_move_rules(),
        chess_set::PieceType::King => king::get_king_move_rules(),
    }
}
