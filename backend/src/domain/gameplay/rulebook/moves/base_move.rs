use crate::domain::gameplay::chess_set;

pub trait ChessMove {
    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError>;
}
