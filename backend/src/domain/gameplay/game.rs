use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook;

enum Command {
    MakeStandardMove {
        from_square: chess_set::Square,
        to_square: chess_set::Square,
    },
}

#[derive(thiserror::Error, Debug, PartialEq)]
enum CommandHandlingError {
    #[error("{0}")]
    ChessboardActionError(chess_set::ChessboardActionError),

    #[error("The game has already ended.")]
    GameHasAlreadyEnded,

    #[error("Move is out of turn - it's currently {0}'s turn.")]
    PlayIsOutOfTurn(chess_set::Colour),

    #[error("{0}")]
    MoveValidationError(rulebook::MoveValidationError),
}

#[derive(Debug, PartialEq)]
enum GameStatus {
    ToPlay(chess_set::Colour),
    Won(chess_set::Colour),
    Drawn,
}

/// Event sourced representation of a game of chess.
struct Game {
    chessboard: chess_set::Chessboard,
    status: GameStatus,
    command_history: Vec<Command>,
}

impl Game {
    // Factories.

    pub fn new() -> Self {
        let starting_position = rulebook::get_official_starting_position();
        let chessboard = chess_set::Chessboard::new(starting_position);

        Self {
            chessboard: chessboard,
            command_history: vec![],
            status: GameStatus::ToPlay(chess_set::Colour::White),
        }
    }

    // Queries.

    fn get_piece_at_square(&self, square: &chess_set::Square) -> Option<chess_set::Piece> {
        self.chessboard.get_piece(square)
    }

    // Mutators.

    pub fn handle_command(
        &mut self,
        command: Command,
    ) -> Result<&GameStatus, CommandHandlingError> {
        let GameStatus::ToPlay(to_play_colour) = self.status else {
            return Err(CommandHandlingError::GameHasAlreadyEnded);
        };

        if let Err(handling_error) = match command {
            Command::MakeStandardMove {
                from_square,
                to_square,
            } => self.make_move(&from_square, &to_square, &to_play_colour),
        } {
            return Err(handling_error);
        }

        self.command_history.push(command);
        self.progress_game_status(to_play_colour);
        Ok(&self.status)
    }

    fn progress_game_status(&mut self, just_played_colour: chess_set::Colour) {
        // TODO - check for win / draw using rulebook.
        self.status = GameStatus::ToPlay(just_played_colour.swap());
    }

    fn make_move(
        &mut self,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
        to_play_colour: &chess_set::Colour,
    ) -> Result<(), CommandHandlingError> {
        // Check the move isn't out of turn.
        let Some(piece) = self.get_piece_at_square(&from_square) else {
            return Err(CommandHandlingError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(from_square.clone()),
            ));
        };
        if !(piece.get_colour() == to_play_colour) {
            return Err(CommandHandlingError::PlayIsOutOfTurn(*to_play_colour));
        };

        // Validate the move against the rules and move the piece.
        if let Err(error) =
            rulebook::validate_move(&self.chessboard, &piece, &from_square, &to_square)
        {
            return Err(CommandHandlingError::MoveValidationError(error));
        };
        match self.chessboard.move_piece(&from_square, &to_square) {
            Ok(()) => Ok(()),
            Err(error) => Err(CommandHandlingError::ChessboardActionError(error)),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod command_handler_tests {
        use super::super::*;

        #[test]
        fn can_make_1e4_opening() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(chess_set::Rank::TWO, chess_set::File::E);
            let to_square = chess_set::Square::new(chess_set::Rank::FOUR, chess_set::File::E);
            let opening_move = Command::MakeStandardMove {
                from_square,
                to_square,
            };

            let result = game.handle_command(opening_move);

            assert_eq!(result, Ok(&GameStatus::ToPlay(chess_set::Colour::Black)));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            assert_ne!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_made_by_black() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(chess_set::Rank::SEVEN, chess_set::File::C);
            let to_square = chess_set::Square::new(chess_set::Rank::SIX, chess_set::File::C);
            let opening_move = Command::MakeStandardMove {
                from_square,
                to_square,
            };

            let result = game.handle_command(opening_move);

            let expected_error = CommandHandlingError::PlayIsOutOfTurn(chess_set::Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_from_empty_square() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(chess_set::Rank::THREE, chess_set::File::H);
            let to_square = chess_set::Square::new(chess_set::Rank::FOUR, chess_set::File::H);
            let opening_move = Command::MakeStandardMove {
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
