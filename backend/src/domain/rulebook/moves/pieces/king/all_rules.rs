use super::castle::AllowCastle;
use super::single_square_any_direction::AllowSingleSquareAnyDirection;
use crate::domain::rulebook::moves::chess_move;
use std::vec;

pub fn get_king_move_rules() -> vec::IntoIter<Box<dyn chess_move::MoveRule>> {
    vec![
        Box::new(AllowSingleSquareAnyDirection) as Box<dyn chess_move::MoveRule>,
        Box::new(AllowCastle) as Box<dyn chess_move::MoveRule>,
    ]
    .into_iter()
}
