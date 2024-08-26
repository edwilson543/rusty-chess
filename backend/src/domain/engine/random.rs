use super::engine::ChessEngine;
use crate::domain::game;
use crate::domain::rulebook;

pub struct Random;

impl ChessEngine for Random {
    fn suggest_next_move(game: &game::Game) -> Box<dyn rulebook::Move> {
        todo!()
    }
}
