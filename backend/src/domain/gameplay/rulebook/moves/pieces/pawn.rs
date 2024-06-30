use super::super::{rule, translation};
use crate::domain::gameplay::rulebook::moves::move_;
use std::vec;

pub fn get_pawn_move_rules() -> vec::IntoIter<Box<dyn rule::Rule>> {
    let one_square_forwards_rule = rule::SingleSquareMove::new(translation::ChessVector::new(0, 1));

    let rules = vec![
        Box::new(one_square_forwards_rule) as Box<dyn rule::Rule>,
        Box::new(TwoSquaresForwardTranslation) as Box<dyn rule::Rule>,
        // TODO -> en_passant.
    ];

    rules.into_iter()
}

struct TwoSquaresForwardTranslation;

// TODO -> restrict this to the Pawn's first move.
impl rule::Rule for TwoSquaresForwardTranslation {
    fn allows_move(&self, move_: &move_::Move) -> bool {
        let forwards = translation::ChessVector::new(0, 1);
        move_.translation.vector == forwards && move_.translation.scalar == 2
    }
}
