use chess::domain::gameplay::chess_set::{Colour, File, Rank, Square};
use chess::domain::gameplay::{Game, GameError};

#[test]
fn black_cannot_move_pawn_pinned_by_white_queen() {
    let mut game = Game::new();

    let from_square = Square::new(Rank::Two, File::E);
    let to_square = Square::new(Rank::Four, File::E);
    let _ = game
        .play_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::E);
    let to_square = Square::new(Rank::Five, File::E);
    let _ = game
        .play_ordinary_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::One, File::D);
    let to_square = Square::new(Rank::Five, File::H);
    let _ = game
        .play_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::F);
    let captured_piece_square = Square::new(Rank::Six, File::F);
    let invalid_move =
        game.play_ordinary_move(&Colour::Black, &from_square, &captured_piece_square);

    let expected_error = Err(GameError::MoveWouldLeavePlayerInCheck);

    assert_eq!(invalid_move, expected_error)
}
