use super::super::{translation_rule, translations};
use crate::domain::gameplay::chess_set;

pub fn get_translation_rules_for_piece(
    piece_type: &chess_set::PieceType,
) -> Vec<translation_rule::TranslationRule> {
    panic!("todo")
    // match piece_type {
    //     chess_set::PieceType::Pawn => PAWN_TRANSLATION_RULES,
    //     chess_set::PieceType::Knight => KNIGHT_TRANSLATION_RULES,
    //     chess_set::PieceType::Bishop => BISHOP_TRANSLATION_RULES,
    //     chess_set::PieceType::Rook => ROOK_TRANSLATION_RULES,
    //     chess_set::PieceType::Queen => QUEEN_TRANSLATION_RULES,
    //     chess_set::PieceType::King => KING_TRANSLATION_RULES,
    // }
}
