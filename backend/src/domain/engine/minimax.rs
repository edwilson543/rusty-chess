use super::engine;
use crate::domain::engine::evaluation;
use crate::domain::{chess_set, game, rulebook};
use std::cmp;

const MAX_SEARCH_DEPTH: u8 = 3;

pub struct Minimax {
    evaluator: Box<dyn evaluation::ChessboardEvaluator>,
}

impl engine::ChessEngine for Minimax {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook::Move, engine::SuggestNextMoveError> {
        let Some(to_play_colour) = game.get_status().to_play_colour() else {
            return Err(engine::SuggestNextMoveError::GameHasAlreadyEnded);
        };

        let (_, maybe_move) = self.minimax(
            &to_play_colour,
            game.clone(),
            MAX_SEARCH_DEPTH,
            i32::MIN,
            i32::MAX,
        );

        Ok(maybe_move.unwrap())
    }
}

impl Minimax {
    pub fn new(evaluator: Box<dyn evaluation::ChessboardEvaluator>) -> Self {
        Self { evaluator }
    }

    fn minimax(
        &self,
        maximizer: &chess_set::Colour,
        game: game::Game,
        max_search_depth: u8,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, Option<rulebook::Move>) {
        let status = game.get_status();

        // Evaluate for a terminal game.
        if status == &game::GameStatus::Drawn {
            return (0, None);
        };
        match status.winner() {
            Some(colour) => match &colour == maximizer {
                true => return (i32::MAX, None),
                false => return (i32::MIN, None),
            },
            None => {}
        }

        if max_search_depth == 0 {
            let current_chessboard = game.current_chessboard().clone();
            let score = self
                .evaluator
                .evaluate_position(current_chessboard, maximizer);
            return (score, None);
        };

        let to_play_colour = status.to_play_colour().unwrap();
        let is_maximizers_move = &to_play_colour == maximizer;

        let mut current_best_move: Option<rulebook::Move> = None;
        let mut current_best_score = match is_maximizers_move {
            true => i32::MIN,
            false => i32::MAX,
        };

        for chess_move in rulebook::get_legal_moves(to_play_colour, game.get_chessboard_history()) {
            let mut deeper_game = game.clone();
            deeper_game.play_validated_move(chess_move.clone()).unwrap();
            let (maybe_better_score, _) =
                self.minimax(maximizer, deeper_game, max_search_depth - 1, alpha, beta);

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
}
