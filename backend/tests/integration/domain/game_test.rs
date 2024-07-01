use chess::domain::gameplay::chess_set::{Colour, File, Rank, Square};
use chess::domain::gameplay::{Game, GameStatus};

#[test]
fn white_can_make_en_passant() {
    let mut game = Game::new();

    let from_square = Square::new(Rank::Two, File::A);
    let to_square = Square::new(Rank::Four, File::A);
    let _ = game
        .make_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::H);
    let to_square = Square::new(Rank::Six, File::H);
    let _ = game
        .make_ordinary_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Four, File::A);
    let to_square = Square::new(Rank::Five, File::A);
    let _ = game
        .make_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::B);
    let captured_piece_square = Square::new(Rank::Five, File::B);
    let _ = game
        .make_ordinary_move(&Colour::Black, &from_square, &captured_piece_square)
        .unwrap();

    let from_square = Square::new(Rank::Five, File::A);
    let to_square = Square::new(Rank::Six, File::B);
    let result = game.make_en_passant(&Colour::White, &from_square, &to_square);

    assert_eq!(result, Ok(&GameStatus::ToPlay(Colour::Black)));
    assert_eq!(game.get_piece_at_square(&from_square), None);
    assert_ne!(game.get_piece_at_square(&to_square), None);
    assert_eq!(game.get_piece_at_square(&captured_piece_square), None);
}
