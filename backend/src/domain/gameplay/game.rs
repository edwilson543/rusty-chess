use crate::domain::gameplay::chess_set;
use crate::domain::gameplay::rulebook;

pub enum GameMove {
    OrdinaryMove(rulebook::OrdinaryMove),
    EnPassant(rulebook::EnPassant),
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum GameError {
    #[error("The game has already ended.")]
    GameHasAlreadyEnded,

    #[error("Move is out of turn - it's currently {0}'s turn.")]
    PlayIsOutOfTurn(chess_set::Colour),

    #[error("{0} player attempted to move opponent's piece.")]
    CannotMoveOpponentPiece(chess_set::Colour),

    #[error("{0}")]
    MoveValidationError(rulebook::MoveValidationError),

    #[error("{0}")]
    EnPassantValidationError(rulebook::EnPassantValidationError),

    #[error("{0}")]
    ChessboardActionError(chess_set::ChessboardActionError),
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    ToPlay(chess_set::Colour),
    Won(chess_set::Colour),
    Drawn,
}

/// Event sourced representation of a game of chess.
pub struct Game {
    chessboard: chess_set::Chessboard,
    status: GameStatus,
    history: Vec<GameMove>,
}

// Public interface.
impl Game {
    pub fn new() -> Self {
        let starting_position = rulebook::get_official_starting_position();
        let chessboard = chess_set::Chessboard::new(starting_position);

        Self {
            chessboard: chessboard,
            history: vec![],
            status: GameStatus::ToPlay(chess_set::Colour::White),
        }
    }

    pub fn play_ordinary_move(
        &mut self,
        player: &chess_set::Colour,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Result<&GameStatus, GameError> {
        if let Err(error) = self.check_if_play_is_out_of_turn(player) {
            return Err(error);
        }

        let piece = match self.check_piece_at_square_belongs_to_player(player, from_square) {
            Ok(piece) => piece,
            Err(error) => return Err(error),
        };

        let validated_move =
            match rulebook::validate_move(&self.chessboard, &piece, &from_square, &to_square) {
                Ok(validated_move) => validated_move,
                Err(error) => return Err(GameError::MoveValidationError(error)),
            };

        match self.chessboard.move_piece(&from_square, &to_square) {
            Err(error) => return Err(GameError::ChessboardActionError(error)),
            Ok(()) => {}
        };

        self.history.push(GameMove::OrdinaryMove(validated_move));
        self.progress_game_status();
        Ok(&self.status)
    }

    pub fn play_en_passant(
        &mut self,
        player: &chess_set::Colour,
        from_square: &chess_set::Square,
        to_square: &chess_set::Square,
    ) -> Result<&GameStatus, GameError> {
        if let Err(error) = self.check_if_play_is_out_of_turn(player) {
            return Err(error);
        }

        let pawn = match self.check_piece_at_square_belongs_to_player(player, from_square) {
            Ok(pawn) => pawn,
            Err(error) => return Err(error),
        };

        let GameMove::OrdinaryMove(previous_move) = self.history.last().unwrap() else {
            return Err(GameError::EnPassantValidationError(
                rulebook::EnPassantValidationError::OnlyAllowedAfterDoubleAdvancement,
            ));
        };

        let en_passant =
            match rulebook::validate_en_passant(&pawn, from_square, to_square, previous_move) {
                Ok(en_passant) => en_passant,
                Err(error) => return Err(GameError::EnPassantValidationError(error)),
            };

        match self.chessboard.remove_piece(&previous_move.to_square) {
            Err(error) => return Err(GameError::ChessboardActionError(error)),
            Ok(_) => {}
        };
        match self.chessboard.move_piece(&from_square, &to_square) {
            Err(error) => return Err(GameError::ChessboardActionError(error)),
            Ok(()) => {}
        };

        self.history.push(GameMove::EnPassant(en_passant));
        self.progress_game_status();
        Ok(&self.status)
    }
}

// Private interface.
impl Game {
    // Mutators.
    fn progress_game_status(&mut self) {
        // TODO - check for win / draw using rulebook.
        self.status = match self.status {
            GameStatus::ToPlay(colour) => GameStatus::ToPlay(colour.swap()),
            _ => panic!("TODO."),
        };
    }

    // Queries.
    pub fn get_piece_at_square(&self, square: &chess_set::Square) -> Option<chess_set::Piece> {
        self.chessboard.get_piece(square)
    }

    fn check_if_play_is_out_of_turn(&self, player: &chess_set::Colour) -> Result<(), GameError> {
        let GameStatus::ToPlay(to_play_colour) = self.status else {
            return Err(GameError::GameHasAlreadyEnded);
        };
        if !(player == &to_play_colour) {
            return Err(GameError::PlayIsOutOfTurn(to_play_colour.clone()));
        };
        Ok(())
    }

    fn check_piece_at_square_belongs_to_player(
        &self,
        player: &chess_set::Colour,
        square: &chess_set::Square,
    ) -> Result<chess_set::Piece, GameError> {
        let Some(piece) = self.get_piece_at_square(square) else {
            return Err(GameError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(square.clone()),
            ));
        };
        if !(piece.get_colour() == player) {
            return Err(GameError::CannotMoveOpponentPiece(*player));
        };
        Ok(piece)
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod play_ordinary_move_tests {
        use super::super::*;
        use crate::domain::gameplay::chess_set::{Colour, File, PieceType, Rank};

        #[test]
        fn can_make_1e4_pawn_opening() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Two, File::E);
            let to_square = chess_set::Square::new(Rank::Four, File::E);

            let result = game.play_ordinary_move(&Colour::White, &from_square, &to_square);

            assert_eq!(result, Ok(&GameStatus::ToPlay(Colour::Black)));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            let moved_pawn = game.get_piece_at_square(&to_square).unwrap();
            assert_eq!(moved_pawn.get_piece_type(), &PieceType::Pawn);
        }

        #[test]
        fn can_make_nf3_knight_opening() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::One, File::G);
            let to_square = chess_set::Square::new(Rank::Three, File::F);

            let result = game.play_ordinary_move(&Colour::White, &from_square, &to_square);

            assert_eq!(result, Ok(&GameStatus::ToPlay(Colour::Black)));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            let moved_knight = game.get_piece_at_square(&to_square).unwrap();
            assert_eq!(moved_knight.get_piece_type(), &PieceType::Knight);
        }

        #[test]
        fn errors_for_opening_made_by_black() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);

            let result = game.play_ordinary_move(&Colour::Black, &from_square, &to_square);

            let expected_error = GameError::PlayIsOutOfTurn(Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_attempt_to_move_opponents_piece() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);

            let result = game.play_ordinary_move(&Colour::White, &from_square, &to_square);

            let expected_error = GameError::CannotMoveOpponentPiece(Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_from_empty_square() {
            let mut game = Game::new();

            let from_square = chess_set::Square::new(Rank::Three, File::H);
            let to_square = chess_set::Square::new(Rank::Four, File::H);

            let result = game.play_ordinary_move(&Colour::White, &from_square, &to_square);

            let expected_error = GameError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(from_square),
            );
            assert_eq!(result, Err(expected_error));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }
    }
}
