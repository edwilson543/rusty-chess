use crate::domain::gameplay::rulebook::OrdinaryMove;

/// Mechanism for defining whether a certain translation is allowed.
pub trait OrdinaryMoveRule {
    fn allows_move(&self, chess_move: &OrdinaryMove) -> bool;
}
