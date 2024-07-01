use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook;

pub enum Move {
    OrdinaryMove {
        player: chess_set::Colour,
        from_square: chess_set::Square,
        to_square: chess_set::Square,
    },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MoveError {
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

#[derive(Debug, PartialEq)]
enum GameStatus {
    ToPlay(chess_set::Colour),
    Won(chess_set::Colour),
    Drawn,
}

/// Event sourced representation of a game of chess.
pub struct Game {
    chessboard: chess_set::Chessboard,
    status: GameStatus,
    move_history: Vec<Move>,
}

// Public interface.
impl Game {
    pub fn new() -> Self {
        let starting_position = rulebook::get_official_starting_position();
        let chessboard = chess_set::Chessboard::new(starting_position);

        Self {
            chessboard: chessboard,
            move_history: vec![],
            status: GameStatus::ToPlay(chess_set::Colour::White),
        }
    }

    pub fn make_move(&mut self, chess_move: Move) -> Result<&GameStatus, MoveError> {
        if let Err(handling_error) = match chess_move {
            Move::OrdinaryMove {
                player,
                from_square,
                to_square,
            } => self.make_ordinary_move(&player, &from_square, &to_square),
        } {
            return Err(handling_error);
        }

        self.move_history.push(chess_move);
        self.progress_game_status();
        Ok(&self.status)
    }
}

// Private interface.
impl Game {
    fn get_piece_at_square(&self, square: &chess_set::Square) -> Option<chess_set::Piece> {
        self.chessboard.get_piece(square)
    }

    fn progress_game_status(&mut self) {
        // TODO - check for win / draw using rulebook.
        self.status = match self.status {
            GameStatus::ToPlay(colour) => GameStatus::ToPlay(colour.swap()),
            _ => panic!("TODO."),
        };
    }

    fn make_ordinary_move(
        &mut self,
        player: &chess_set::Colour,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Result<(), MoveError> {
        // Check the move isn't out of turn.
        let GameStatus::ToPlay(to_play_colour) = self.status else {
            return Err(MoveError::GameHasAlreadyEnded);
        };
        if !(player == &to_play_colour) {
            return Err(MoveError::PlayIsOutOfTurn(to_play_colour.clone()));
        }

        // Check the player is moving a piece of their own colour.
        let Some(piece) = self.get_piece_at_square(&from_square) else {
            return Err(MoveError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(from_square.clone()),
            ));
        };
        if !(piece.get_colour() == &to_play_colour) {
            return Err(MoveError::CannotMoveOpponentPiece(*player));
        };

        // Check the move against the rulebook.
        if let Err(error) =
            rulebook::validate_move(&self.chessboard, &piece, &from_square, &to_square)
        {
            return Err(MoveError::MoveValidationError(error));
        };

        // Finally, move the piece.
        match self.chessboard.move_piece(&from_square, &to_square) {
            Ok(()) => Ok(()),
            Err(error) => Err(MoveError::ChessboardActionError(error)),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod make_move_tests {
        use super::super::*;
        use crate::domain::gameplay::chess_set::{Colour, File, Rank};

        #[test]
        fn can_make_1e4_opening() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Two, File::E);
            let to_square = chess_set::Square::new(Rank::Four, File::E);
            let opening_move = Move::OrdinaryMove {
                player: Colour::White,
                from_square,
                to_square,
            };

            let result = game.make_move(opening_move);

            assert_eq!(result, Ok(&GameStatus::ToPlay(Colour::Black)));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            assert_ne!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_made_by_black() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);
            let opening_move = Move::OrdinaryMove {
                player: Colour::Black,
                from_square,
                to_square,
            };

            let result = game.make_move(opening_move);

            let expected_error = MoveError::PlayIsOutOfTurn(Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_attempt_to_move_opponents_piece() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);
            let opening_move = Move::OrdinaryMove {
                player: Colour::White,
                from_square,
                to_square,
            };

            let result = game.make_move(opening_move);

            let expected_error = MoveError::CannotMoveOpponentPiece(Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_from_empty_square() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Three, File::H);
            let to_square = chess_set::Square::new(Rank::Four, File::H);
            let opening_move = Move::OrdinaryMove {
                player: Colour::White,
                from_square,
                to_square,
            };

            let result = game.make_move(opening_move);

            let expected_error = MoveError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(from_square),
            );
            assert_eq!(result, Err(expected_error));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }
    }
}
