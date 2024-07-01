use super::commands::{Command, CommandHandlingError};
use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook;

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
    command_history: Vec<Command>,
}

// Public interface.
impl Game {
    pub fn new() -> Self {
        let starting_position = rulebook::get_official_starting_position();
        let chessboard = chess_set::Chessboard::new(starting_position);

        Self {
            chessboard: chessboard,
            command_history: vec![],
            status: GameStatus::ToPlay(chess_set::Colour::White),
        }
    }

    pub fn handle_command(
        &mut self,
        command: Command,
    ) -> Result<&GameStatus, CommandHandlingError> {
        if let Err(handling_error) = match command {
            Command::MakeMove {
                player,
                from_square,
                to_square,
            } => self.make_move(&player, &from_square, &to_square),
        } {
            return Err(handling_error);
        }

        self.command_history.push(command);
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

    fn make_move(
        &mut self,
        player: &chess_set::Colour,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Result<(), CommandHandlingError> {
        // Check the move isn't out of turn.
        let GameStatus::ToPlay(to_play_colour) = self.status else {
            return Err(CommandHandlingError::GameHasAlreadyEnded);
        };
        if !(player == &to_play_colour) {
            return Err(CommandHandlingError::PlayIsOutOfTurn(
                to_play_colour.clone(),
            ));
        }

        // Check the player is moving a piece of their own colour.
        let Some(piece) = self.get_piece_at_square(&from_square) else {
            return Err(CommandHandlingError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(from_square.clone()),
            ));
        };
        if !(piece.get_colour() == &to_play_colour) {
            return Err(CommandHandlingError::CannotMoveOpponentPiece(*player));
        };

        // Check the move against the rulebook.
        if let Err(error) =
            rulebook::validate_move(&self.chessboard, &piece, &from_square, &to_square)
        {
            return Err(CommandHandlingError::MoveValidationError(error));
        };

        // Finally, move the piece.
        match self.chessboard.move_piece(&from_square, &to_square) {
            Ok(()) => Ok(()),
            Err(error) => Err(CommandHandlingError::ChessboardActionError(error)),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod handle_command_tests {
        use super::super::*;
        use crate::domain::gameplay::chess_set::{Colour, File, Rank};

        #[test]
        fn can_make_1e4_opening() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Two, File::E);
            let to_square = chess_set::Square::new(Rank::Four, File::E);
            let opening_move = Command::MakeMove {
                player: Colour::White,
                from_square,
                to_square,
            };

            let result = game.handle_command(opening_move);

            assert_eq!(result, Ok(&GameStatus::ToPlay(Colour::Black)));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            assert_ne!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_made_by_black() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);
            let opening_move = Command::MakeMove {
                player: Colour::Black,
                from_square,
                to_square,
            };

            let result = game.handle_command(opening_move);

            let expected_error = CommandHandlingError::PlayIsOutOfTurn(Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_attempt_to_move_opponents_piece() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);
            let opening_move = Command::MakeMove {
                player: Colour::White,
                from_square,
                to_square,
            };

            let result = game.handle_command(opening_move);

            let expected_error = CommandHandlingError::CannotMoveOpponentPiece(Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_from_empty_square() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Three, File::H);
            let to_square = chess_set::Square::new(Rank::Four, File::H);
            let opening_move = Command::MakeMove {
                player: Colour::White,
                from_square,
                to_square,
            };

            let result = game.handle_command(opening_move);

            let expected_error = CommandHandlingError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(from_square),
            );
            assert_eq!(result, Err(expected_error));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }
    }
}
