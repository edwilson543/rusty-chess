use crate::domain::chess_set;

use super::insufficient_material;

pub enum Draw {
    InsufficientMaterial,
}

pub fn is_draw(chessboard_history: &Vec<chess_set::Chessboard>) -> Option<Draw> {
    if insufficient_material::is_insufficient_material_draw(chessboard_history.last().unwrap()) {
        return Some(Draw::InsufficientMaterial);
    };

    None
}
