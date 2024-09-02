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
            let boxed_move = Box::new(ordinary_move) as Box<dyn Move>;

            if let Ok(()) = &boxed_move.validate(chessboard_history) {
                let Ok(left_in_check) =
                    check::would_player_be_left_in_check(&player, &boxed_move, &chessboard)
                else {
                    continue;
                };
                if !left_in_check {
                    legal_moves.push(boxed_move)
                }
            }
        }

        // Add any valid special moves.
        maybe_append_en_passant(
            &moved_piece,
            &from_square,
            chessboard_history,
            &mut legal_moves,
        )
    }

    legal_moves
}

fn maybe_append_en_passant(
    piece: &chess_set::Piece,
    from_square: &chess_set::Square,
    chessboard_history: &Vec<chess_set::Chessboard>,
    legal_moves: &mut Vec<Box<dyn Move>>,
) {
    if !(piece.get_piece_type() == &chess_set::PieceType::Pawn) {
        return;
    }

    let forwards_one = moves::ChessVector::forwards(piece.get_colour());
    let to_square = forwards_one.apply_to_square(&from_square);

    let en_passant = moves::EnPassant::new(piece, from_square, &to_square);

    match en_passant.validate(chessboard_history) {
        Ok(()) => legal_moves.push(Box::new(en_passant) as Box<dyn Move>),
        Err(_) => {}
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
