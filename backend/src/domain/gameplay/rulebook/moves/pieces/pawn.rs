use super::super::{common, move_rule, translation};
use std::vec;

pub fn get_pawn_move_rules() -> vec::IntoIter<Box<dyn move_rule::MoveRule>> {
    let one_square_forwards_rule =
        common::SingleSquareMove::new(translation::ChessVector::new(0, 1));

    let rules = vec![
        Box::new(one_square_forwards_rule) as Box<dyn move_rule::MoveRule>,
        Box::new(TwoSquaresForwardTranslation) as Box<dyn move_rule::MoveRule>,
        // TODO -> en_passant.
    ];

    rules.into_iter()
}

struct TwoSquaresForwardTranslation;

// TODO -> restrict this to the Pawn's first move.
impl move_rule::MoveRule for TwoSquaresForwardTranslation {
    fn allows_move(&self, move_: &move_rule::Move) -> bool {
        let forwards = translation::ChessVector::new(0, 1);
        move_.translation.vector == forwards && move_.translation.scalar == 2
    }
}
