use crate::domain::chess_set;
use crate::domain::rulebook::{check, moves, Move};

pub fn get_legal_moves(
    player: chess_set::Colour,
    chessboard: &chess_set::Chessboard,
) -> Vec<Box<dyn Move>> {
    // TODO -> include special moves - will require passing in history.
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

            // TODO -> use actual chessboard history here.
            if let Ok(()) = &boxed_move.validate(&vec![chessboard.clone()]) {
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
    }

    legal_moves
}

#[cfg(test)]
mod tests {
    use super::get_legal_moves;
    use crate::domain::chess_set::Colour;
    use crate::testing::factories;

    #[test]
    fn there_are_twenty_legal_opening_moves() {
        let chessboard = factories::chessboard();

        let legal_moves = get_legal_moves(Colour::White, &chessboard);

        assert_eq!(legal_moves.len(), 20);
    }
}
