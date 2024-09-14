use super::engine;
use crate::domain::engine::evaluation;
use crate::domain::{chess_set, game, rulebook};
use std::cmp;

/// Classical style chess engine that uses minimax with alpha-beta pruning.
pub struct Minimax {
    evaluator: Box<dyn evaluation::ChessboardEvaluator>,
    max_search_depth: u8,
}

impl engine::ChessEngine for Minimax {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook::Move, engine::SuggestNextMoveError> {
        let Some(to_play_colour) = game.get_status().to_play_colour() else {
            return Err(engine::SuggestNextMoveError::GameHasAlreadyEnded);
        };

        let (_, maybe_move) = self.minimax(&to_play_colour, game.clone(), 0, i32::MIN, i32::MAX);

        match maybe_move {
            Some(chess_move) => Ok(chess_move),
            None => panic!("Minimax di not generate a move!"),
        }
    }
}

impl Minimax {
    pub fn new(evaluator: Box<dyn evaluation::ChessboardEvaluator>, max_search_depth: u8) -> Self {
        Self {
            evaluator,
            max_search_depth,
        }
    }

    fn minimax(
        &self,
        maximizer: &chess_set::Colour,
        game: game::Game,
        current_search_depth: u8,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, Option<rulebook::Move>) {
        if let Some(terminal_score) =
            self.evaluate_if_should_stop_searching(maximizer, &game, current_search_depth)
        {
            return (terminal_score, None);
        }

        let to_play_colour = game.get_status().to_play_colour().unwrap();
        let is_maximizers_move = &to_play_colour == maximizer;

        let mut current_best_move: Option<rulebook::Move> = None;
        let mut current_best_score = match is_maximizers_move {
            true => i32::MIN,
            false => i32::MAX,
        };

        for chess_move in rulebook::get_legal_moves(to_play_colour, game.get_chessboard_history()) {
            let mut deeper_game = game.clone();
            deeper_game.play_validated_move(&chess_move).unwrap();

            let (maybe_better_score, _) = self.minimax(
                maximizer,
                deeper_game,
                current_search_depth + 1,
                alpha,
                beta,
            );

            if is_maximizers_move {
                if maybe_better_score > current_best_score {
                    current_best_score = maybe_better_score;
                    current_best_move = Some(chess_move);
                }
                alpha = cmp::max(alpha, current_best_score);

                if alpha >= beta {
                    // Prune.
                    return (current_best_score, current_best_move);
                }
            } else {
                if maybe_better_score < current_best_score {
                    current_best_score = maybe_better_score;
                    current_best_move = Some(chess_move);
                }
                beta = cmp::min(beta, current_best_score);

                if beta <= alpha {
                    // Prune.
                    return (current_best_score, current_best_move);
                }
            }
        }

        (current_best_score, current_best_move)
    }

    fn evaluate_if_should_stop_searching(
        &self,
        maximizer: &chess_set::Colour,
        game: &game::Game,
        current_search_depth: u8,
    ) -> Option<i32> {
        let game_status = game.get_status();

        // Evaluate a drawn game.
        if game_status.is_draw() {
            return Some(0);
        };

        // Evaluate a won game.
        match game_status.winner() {
            Some(colour) => {
                return match &colour == maximizer {
                    // Current search depth is included to reward a quicker win / slower loss.
                    true => Some(i32::MAX - (current_search_depth as i32)),
                    false => Some(i32::MIN + (current_search_depth as i32)),
                };
            }
            None => {}
        }

        // Evaluate the position if we're at the max search depth.
        if current_search_depth == self.max_search_depth {
            let current_chessboard = game.current_chessboard().clone();
            let score = self
                .evaluator
                .evaluate_position(current_chessboard, maximizer);
            return Some(score);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Minimax;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::engine::{evaluation, ChessEngine};
    use crate::domain::game::{Game, GameStatus};
    use std::collections::BTreeMap;

    #[test]
    fn minimax_completes_fools_mate_for_black() {
        let mut game = Game::new(1);

        // Set up a Fool's mate.
        let from_square = Square::new(Rank::Two, File::F);
        let to_square = Square::new(Rank::Three, File::F);
        let _ = game
            .play_move(&Colour::White, &from_square, &to_square)
            .unwrap();

        let from_square = Square::new(Rank::Seven, File::E);
        let to_square = Square::new(Rank::Six, File::E);
        let _ = game
            .play_move(&Colour::Black, &from_square, &to_square)
            .unwrap();

        let from_square = Square::new(Rank::Two, File::G);
        let to_square = Square::new(Rank::Four, File::G);
        let _ = game
            .play_move(&Colour::White, &from_square, &to_square)
            .unwrap();

        // Test whether the minimax engine can complete the fool's mate.
        let evaluator = evaluation::PiecePlacementChessboardEvaluator;
        let minimax_engine = Minimax::new(Box::new(evaluator), 1);

        let generated_move = minimax_engine.generate_next_move(&game).unwrap();

        assert_eq!(generated_move.from_square.get_rank(), &Rank::Eight);
        assert_eq!(generated_move.from_square.get_file(), &File::D);
        assert_eq!(generated_move.to_square.get_rank(), &Rank::Four);
        assert_eq!(generated_move.to_square.get_file(), &File::H);

        game.play_validated_move(&generated_move).unwrap();

        assert_eq!(game.get_status(), &GameStatus::WonByBlack);
    }

    #[test]
    fn minimax_delivers_three_move_back_rank_mate() {
        let mut starting_position = BTreeMap::new();

        // Set up a back rank mate on white.
        let white_king = Piece::new(Colour::White, PieceType::King);
        let white_king_square = Square::new(Rank::One, File::A);
        starting_position.insert(white_king_square, white_king);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        let black_rook_square = Square::new(Rank::Eight, File::F);
        starting_position.insert(black_rook_square, black_rook);

        let black_queen = Piece::new(Colour::Black, PieceType::Queen);
        let black_queen_square = Square::new(Rank::Seven, File::E);
        starting_position.insert(black_queen_square, black_queen);

        let chessboard = Chessboard::new(starting_position);
        let mut game = Game::reincarnate(1, GameStatus::ToPlayBlack, vec![chessboard]);

        let evaluator = evaluation::PiecePlacementChessboardEvaluator;
        // Allow a search depth of 3, so that black can initially see the guaranteed mate.
        let minimax_engine = Minimax::new(Box::new(evaluator), 3);

        let black_setup_move = minimax_engine.generate_next_move(&game).unwrap();
        game.play_validated_move(&black_setup_move).unwrap();

        let white_move = minimax_engine.generate_next_move(&game).unwrap();
        game.play_validated_move(&white_move).unwrap();

        let black_checkmate_move = minimax_engine.generate_next_move(&game).unwrap();
        game.play_validated_move(&black_checkmate_move).unwrap();

        assert_eq!(game.get_status(), &GameStatus::WonByBlack);
    }

    #[test]
    fn minimax_captures_opponent_queen() {
        let mut starting_position = BTreeMap::new();

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        // Put the white queen on a square next to the black king.
        let white_queen = Piece::new(Colour::White, PieceType::Queen);
        let white_queen_square = Square::new(Rank::Eight, File::B);
        starting_position.insert(white_queen_square, white_queen);

        let white_king = Piece::new(Colour::White, PieceType::King);
        let white_king_square = Square::new(Rank::One, File::A);
        starting_position.insert(white_king_square, white_king);

        let chessboard = Chessboard::new(starting_position);
        let game = Game::reincarnate(1, GameStatus::ToPlayBlack, vec![chessboard]);

        let evaluator = evaluation::PiecePlacementChessboardEvaluator;
        let minimax_engine = Minimax::new(Box::new(evaluator), 2);

        let generated_move = minimax_engine.generate_next_move(&game).unwrap();

        assert_eq!(generated_move.from_square, black_king_square);
        assert_eq!(generated_move.to_square, white_queen_square);
    }
}
