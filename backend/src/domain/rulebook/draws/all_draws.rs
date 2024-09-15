use crate::domain::chess_set;

use super::{insufficient_material, stalemate};

pub enum Draw {
    InsufficientMaterial,
    Stalemate,
}

pub fn is_draw(
    to_play_colour: chess_set::Colour,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> Option<Draw> {
    if insufficient_material::is_insufficient_material_draw(chessboard_history.last().unwrap()) {
        return Some(Draw::InsufficientMaterial);
    };
    if stalemate::is_stalemate(to_play_colour, chessboard_history) {
        return Some(Draw::Stalemate);
    };

    None
}
