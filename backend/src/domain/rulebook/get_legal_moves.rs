use crate::domain::chess_set;
use crate::domain::rulebook::{check, moves, Move};

/// Get the legal moves that can be played on the latest chessboard in a chessboard history.
///
/// This is used for:
/// * Working out whether a player is checkmated
/// * Generating moves
pub fn get_legal_moves(
    player: chess_set::Colour,
    chessboard_history: &Vec<chess_set::Chessboard>,
) -> Vec<Box<dyn Move>> {
    let chessboard = chessboard_history.last().unwrap();

    let mut legal_moves = vec![];
    for (from_square, maybe_piece) in chessboard.position.clone().into_iter() {
        let Some(moved_piece) = maybe_piece else {
            continue;
        };
        if !(moved_piece.get_colour() == &player) {
            continue;
        };

        for (to_square, _) in chessboard.position.clone().into_iter() {
            let ordinary_move =
                moves::OrdinaryMove::new(chessboard, &moved_piece, &from_square, &to_square);
            push_move_if_legal(
                Box::new(ordinary_move) as Box<dyn Move>,
                &player,
                chessboard_history,
                &mut legal_moves,
            );

            // Add any valid special moves.
            if moved_piece.get_piece_type() == &chess_set::PieceType::Pawn {
                // The obtuseness here is a trade-off:
                // - Avoid re-encoding the rules of en passant (the implemented option)
                // - Consider fewer `to_square`s
                let en_passant = moves::EnPassant::new(&moved_piece, &from_square, &to_square);
                push_move_if_legal(
                    Box::new(en_passant) as Box<dyn Move>,
                    &player,
                    chessboard_history,
                    &mut legal_moves,
                );
            }
        }
    }

    legal_moves
}

fn push_move_if_legal(
    chess_move: Box<dyn Move>,
    player: &chess_set::Colour,
    chessboard_history: &Vec<chess_set::Chessboard>,
    legal_moves: &mut Vec<Box<dyn Move>>,
) {
    match chess_move.validate(chessboard_history) {
        Ok(()) => {}
        Err(_) => return,
    }

    match check::would_player_be_left_in_check(
        &player,
        &chess_move,
        chessboard_history.last().unwrap(),
    ) {
        Ok(left_in_check) => match left_in_check {
            true => return {},
            false => legal_moves.push(chess_move),
        },
        Err(_) => return {},
    }
}

#[cfg(test)]
mod tests {
    use super::get_legal_moves;
    use crate::domain::chess_set::Colour;
    use crate::testing::factories;

    #[test]
    fn there_are_twenty_legal_opening_moves() {
        let chessboard = factories::chessboard();

        let legal_moves = get_legal_moves(Colour::White, &vec![chessboard]);

        assert_eq!(legal_moves.len(), 20);
    }
}
