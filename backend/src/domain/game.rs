use crate::domain::chess_set;
use crate::domain::rulebook;
use serde;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum GameError {
    #[error("The game has already ended.")]
    GameHasAlreadyEnded,

    #[error("{0} player attempted to play out of turn.")]
    PlayIsOutOfTurn(chess_set::Colour),

    #[error("{0} player attempted to move opponent's piece.")]
    CannotMoveOpponentPiece(chess_set::Colour),

    #[error("{0}")]
    MoveValidationErrorV2(rulebook::MoveValidationError),

    #[error("Cannot play move since it would leave player in check.")]
    MoveWouldLeavePlayerInCheck,

    #[error("{0}")]
    ChessboardActionError(chess_set::ChessboardActionError),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum GameStatus {
    ToPlayWhite,
    ToPlayBlack,
    WonByWhite,
    WonByBlack,
    Drawn,
}

/// A single game of chess.
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    id: i32,
    status: GameStatus,
    chessboard_history: Vec<chess_set::Chessboard>,
}

// Public interface.
impl Game {
    pub fn new(id: i32) -> Self {
        let starting_position = rulebook::get_official_starting_position();
        let chessboard = chess_set::Chessboard::new(starting_position);

        Self {
            id: id,
            status: GameStatus::ToPlayWhite,
            chessboard_history: vec![chessboard],
        }
    }

    pub fn reincarnate(
        id: i32,
        status: GameStatus,
        chessboard_history: Vec<chess_set::Chessboard>,
    ) -> Game {
        Self {
            id: id,
            status: status,
            chessboard_history: chessboard_history,
        }
    }

    pub fn play_move(
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

        let chess_move = rulebook::Move::new(piece, from_square.clone(), to_square.clone());

        match rulebook::would_player_be_left_in_check(&chess_move, &self.chessboard_history) {
            Ok(false) => {}
            Ok(true) => return Err(GameError::MoveWouldLeavePlayerInCheck),
            Err(error) => return Err(GameError::MoveValidationErrorV2(error)),
        };

        self.play_validated_move(&chess_move)
    }

    pub fn play_validated_move(
        &mut self,
        chess_move: &rulebook::Move,
    ) -> Result<&GameStatus, GameError> {
        let updated_chessboard = match chess_move.apply_if_valid(&self.chessboard_history) {
            Ok(chessboard) => chessboard,
            Err(error) => return Err(GameError::MoveValidationErrorV2(error)),
        };

        self.chessboard_history.push(updated_chessboard);
        self.progress_game_status();
        Ok(&self.status)
    }

    // Queries.
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_status(&self) -> &GameStatus {
        &self.status
    }

    pub fn get_chessboard_history(&self) -> &Vec<chess_set::Chessboard> {
        &self.chessboard_history
    }

    pub fn current_chessboard(&self) -> &chess_set::Chessboard {
        self.chessboard_history.last().unwrap()
    }
}

// Private interface.
impl Game {
    // Mutators.
    fn progress_game_status(&mut self) {
        let Some(colour) = self.status.to_play_colour() else {
            panic!("Game should have ended sooner!");
        };

        let to_play_colour = colour.swap();

        if rulebook::is_player_checkmated(to_play_colour, self.get_chessboard_history()) {
            self.status = GameStatus::from_winning_colour(colour);
        } else if let Some(_) = rulebook::is_draw(to_play_colour, &self.chessboard_history) {
            self.status = GameStatus::Drawn;
        } else {
            self.status = GameStatus::from_to_play_colour(to_play_colour)
        }
    }

    // Queries.
    pub fn get_piece_at_square(&self, square: &chess_set::Square) -> Option<chess_set::Piece> {
        self.current_chessboard().get_piece(square)
    }

    fn check_if_play_is_out_of_turn(&self, player: &chess_set::Colour) -> Result<(), GameError> {
        let Some(to_play_colour) = self.status.to_play_colour() else {
            return Err(GameError::GameHasAlreadyEnded);
        };
        if !(player == &to_play_colour) {
            return Err(GameError::PlayIsOutOfTurn(to_play_colour.swap()));
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

impl GameStatus {
    pub fn to_play_colour(&self) -> Option<chess_set::Colour> {
        match self {
            GameStatus::ToPlayWhite => Some(chess_set::Colour::White),
            GameStatus::ToPlayBlack => Some(chess_set::Colour::Black),
            _ => None,
        }
    }

    pub fn winner(&self) -> Option<chess_set::Colour> {
        match self {
            GameStatus::WonByWhite => Some(chess_set::Colour::White),
            GameStatus::WonByBlack => Some(chess_set::Colour::Black),
            _ => None,
        }
    }

    pub fn is_draw(&self) -> bool {
        self == &GameStatus::Drawn
    }

    fn from_winning_colour(colour: chess_set::Colour) -> Self {
        match colour {
            chess_set::Colour::White => GameStatus::WonByWhite,
            chess_set::Colour::Black => GameStatus::WonByBlack,
        }
    }

    fn from_to_play_colour(colour: chess_set::Colour) -> Self {
        match colour {
            chess_set::Colour::White => GameStatus::ToPlayWhite,
            chess_set::Colour::Black => GameStatus::ToPlayBlack,
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod play_ordinary_move_tests {
        use super::super::*;
        use crate::domain::chess_set::{Colour, File, PieceType, Rank};

        #[test]
        fn can_make_1e4_pawn_opening() {
            let mut game = Game::new(1);

            let from_square = chess_set::Square::new(Rank::Two, File::E);
            let to_square = chess_set::Square::new(Rank::Four, File::E);

            let result = game.play_move(&Colour::White, &from_square, &to_square);

            assert_eq!(result, Ok(&GameStatus::ToPlayBlack));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            let moved_pawn = game.get_piece_at_square(&to_square).unwrap();
            assert_eq!(moved_pawn.get_piece_type(), &PieceType::Pawn);
        }

        #[test]
        fn can_make_nf3_knight_opening() {
            let mut game = Game::new(1);

            let from_square = chess_set::Square::new(Rank::One, File::G);
            let to_square = chess_set::Square::new(Rank::Three, File::F);

            let result = game.play_move(&Colour::White, &from_square, &to_square);

            assert_eq!(result, Ok(&GameStatus::ToPlayBlack));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            let moved_knight = game.get_piece_at_square(&to_square).unwrap();
            assert_eq!(moved_knight.get_piece_type(), &PieceType::Knight);
        }

        #[test]
        fn errors_for_opening_made_by_black() {
            let mut game = Game::new(1);

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);

            let result = game.play_move(&Colour::Black, &from_square, &to_square);

            let expected_error = GameError::PlayIsOutOfTurn(Colour::Black);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_attempt_to_move_opponents_piece() {
            let mut game = Game::new(1);

            let from_square = chess_set::Square::new(Rank::Seven, File::C);
            let to_square = chess_set::Square::new(Rank::Six, File::C);

            let result = game.play_move(&Colour::White, &from_square, &to_square);

            let expected_error = GameError::CannotMoveOpponentPiece(Colour::White);
            assert_eq!(result, Err(expected_error));
            assert_ne!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_opening_from_empty_square() {
            let mut game = Game::new(1);

            let from_square = chess_set::Square::new(Rank::Three, File::H);
            let to_square = chess_set::Square::new(Rank::Four, File::H);

            let result = game.play_move(&Colour::White, &from_square, &to_square);

            let expected_error = GameError::ChessboardActionError(
                chess_set::ChessboardActionError::SquareIsEmpty(from_square),
            );
            assert_eq!(result, Err(expected_error));
            assert_eq!(game.get_piece_at_square(&from_square), None);
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }

        #[test]
        fn errors_for_illegal_opening_move() {
            let mut game = Game::new(1);

            let from_square = chess_set::Square::new(Rank::One, File::C);
            let to_square = chess_set::Square::new(Rank::Three, File::A);
            let white_bishop = game.get_piece_at_square(&from_square).unwrap();

            let result = game.play_move(&Colour::White, &from_square, &to_square);

            let expected_error = GameError::MoveValidationErrorV2(
                rulebook::MoveValidationError::MoveIsNotLegalForPiece,
            );
            assert_eq!(result, Err(expected_error));
            assert_eq!(game.get_piece_at_square(&from_square), Some(white_bishop));
            assert_eq!(game.get_piece_at_square(&to_square), None);
        }
    }
}
