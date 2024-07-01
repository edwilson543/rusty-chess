use crate::domain::gameplay::{chess_set, rulebook};
use thiserror;

pub enum Command {
    MakeMove {
        player: chess_set::Colour,
        from_square: chess_set::Square,
        to_square: chess_set::Square,
    },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CommandHandlingError {
    #[error("The game has already ended.")]
    GameHasAlreadyEnded,

    #[error("Move is out of turn - it's currently {0}'s turn.")]
    PlayIsOutOfTurn(chess_set::Colour),

    #[error("{0} player attempted to move opponent's piece.")]
    CannotMoveOpponentPiece(chess_set::Colour),

    #[error("{0}")]
    MoveValidationError(rulebook::MoveValidationError),

    #[error("{0}")]
    ChessboardActionError(chess_set::ChessboardActionError),
}
