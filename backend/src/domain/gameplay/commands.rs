use crate::domain::gameplay::{chess_set, rulebook};
use thiserror;

pub enum Command {
    MakeStandardMove {
        from_square: chess_set::Square,
        to_square: chess_set::Square,
    },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CommandHandlingError {
    #[error("{0}")]
    ChessboardActionError(chess_set::ChessboardActionError),

    #[error("The game has already ended.")]
    GameHasAlreadyEnded,

    #[error("Move is out of turn - it's currently {0}'s turn.")]
    PlayIsOutOfTurn(chess_set::Colour),

    #[error("{0}")]
    MoveValidationError(rulebook::MoveValidationError),
}
