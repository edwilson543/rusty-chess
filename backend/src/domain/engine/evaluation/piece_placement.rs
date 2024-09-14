use super::chessboard_evaluator;
use crate::domain::chess_set;

pub struct PiecePlacementChessboardEvaluator;

impl chessboard_evaluator::ChessboardEvaluator for PiecePlacementChessboardEvaluator {
    /// Evaluate the chessboard based on fixed scores assigned to pieces and squares.
    ///
    /// The values are taken from Table 3.1 and 3.1 of  "Neural networks for Chess":
    /// https://github.com/asdfjkl/neural_network_chess/releases
    fn evaluate_position(
        &self,
        chessboard: chess_set::Chessboard,
        for_colour: &chess_set::Colour,
    ) -> i32 {
        let mut total_score = 0;

        for (square, maybe_piece) in chessboard.position.into_iter() {
            let Some(piece) = maybe_piece else { continue };

            let piece_score = evaluate_piece(piece.get_piece_type());
            let square_score = evaluate_square(&square);
            let score = piece_score + square_score;

            match piece.get_colour() == for_colour {
                true => total_score += score,
                false => total_score -= score,
            }
        }

        total_score
    }
}

fn evaluate_square(square: &chess_set::Square) -> i32 {
    let scores = [
        [-50, -40, -30, -30, -30, -30, -40, -50],
        [-40, -20, 0, 0, 0, 0, -20, -40],
        [-30, 0, 10, 15, 15, 10, 0, -30],
        [-30, 5, 15, 20, 20, 15, 05, -30],
        [-30, 5, 15, 20, 20, 15, 05, -30],
        [-30, 0, 10, 15, 15, 10, 0, -30],
        [-40, -20, 0, 0, 0, 0, -20, -40],
        [-50, -40, -30, -30, -30, -30, -40, -50],
    ];
    scores[(square.get_rank().index() - 1) as usize][(square.get_file().index() - 1) as usize]
}

fn evaluate_piece(piece_type: &chess_set::PieceType) -> i32 {
    match piece_type {
        chess_set::PieceType::Pawn => 100,
        chess_set::PieceType::Knight => 310,
        chess_set::PieceType::Bishop => 320,
        chess_set::PieceType::Rook => 500,
        chess_set::PieceType::Queen => 900,
        chess_set::PieceType::King => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::PiecePlacementChessboardEvaluator;
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::engine::evaluation::chessboard_evaluator::ChessboardEvaluator;
    use crate::testing::factories;
    use rstest::rstest;
    use std::collections::BTreeMap;

    #[rstest]
    #[case::white(Colour::White)]
    #[case::black(Colour::Black)]
    fn initial_position_scores_as_zero_for_both_players(#[case] for_colour: Colour) {
        let chessboard = factories::chessboard();

        let score = PiecePlacementChessboardEvaluator.evaluate_position(chessboard, &for_colour);

        assert_eq!(score, 0)
    }

    #[rstest]
    #[case::score_for_white_advantage(Colour::White, 915)]
    #[case::score_for_black_disadvantage(Colour::Black, -915)]
    fn assigns_correct_score_when_white_has_advantage(
        #[case] for_colour: Colour,
        #[case] expected_score: i32,
    ) {
        let mut starting_position = BTreeMap::new();
        let white_queen = Piece::new(Colour::White, PieceType::Queen);
        let square = Square::new(Rank::Three, File::E);
        starting_position.insert(square, white_queen);

        let chessboard = Chessboard::new(starting_position);

        let score = PiecePlacementChessboardEvaluator.evaluate_position(chessboard, &for_colour);

        assert_eq!(score, expected_score)
    }

    #[rstest]
    #[case::score_for_black_advantage(Colour::Black, 745)]
    #[case::score_for_white_disadvantage(Colour::White, -745)]
    fn assigns_correct_score_when_black_has_advantage(
        #[case] for_colour: Colour,
        #[case] expected_score: i32,
    ) {
        let mut starting_position = BTreeMap::new();

        let black_queen = Piece::new(Colour::Black, PieceType::Queen);
        let square = Square::new(Rank::Eight, File::B);
        starting_position.insert(square, black_queen);

        let white_pawn = Piece::new(Colour::White, PieceType::Pawn);
        let square = Square::new(Rank::Three, File::E);
        starting_position.insert(square, white_pawn);

        let chessboard = Chessboard::new(starting_position);

        let score = PiecePlacementChessboardEvaluator.evaluate_position(chessboard, &for_colour);

        assert_eq!(score, expected_score)
    }
}
