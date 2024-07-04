use crate::domain::gameplay::chess_set;

pub trait ChessMove<E> {
    fn apply(
        &self,
        chessboard: &mut chess_set::Chessboard,
    ) -> Result<(), chess_set::ChessboardActionError>;

    fn validate(&self, chessboard_history: &Vec<chess_set::Chessboard>) -> Result<(), E>;
}
