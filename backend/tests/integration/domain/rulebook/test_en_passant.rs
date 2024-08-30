use chess::domain::chess_set::{Colour, File, PieceType, Rank, Square};
use chess::domain::game::{Game, GameStatus};

#[test]
fn white_can_play_move() {
    let mut game = Game::new(1);

    let from_square = Square::new(Rank::Two, File::A);
    let to_square = Square::new(Rank::Four, File::A);
    let _ = game
        .play_unvalidated_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::H);
    let to_square = Square::new(Rank::Six, File::H);
    let _ = game
        .play_unvalidated_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Four, File::A);
    let to_square = Square::new(Rank::Five, File::A);
    let _ = game
        .play_unvalidated_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::B);
    let captured_piece_square = Square::new(Rank::Five, File::B);
    let _ = game
        .play_unvalidated_move(&Colour::Black, &from_square, &captured_piece_square)
        .unwrap();

    let from_square = Square::new(Rank::Five, File::A);
    let to_square = Square::new(Rank::Six, File::B);
    let result = game.play_unvalidated_move(&Colour::White, &from_square, &to_square);

    assert_eq!(result, Ok(&GameStatus::ToPlayBlack));
    assert_eq!(game.get_piece_at_square(&from_square), None);
    assert_eq!(game.get_piece_at_square(&captured_piece_square), None);

    let en_passant_pawn = game.get_piece_at_square(&to_square).unwrap();
    assert_eq!(en_passant_pawn.get_piece_type(), &PieceType::Pawn);
    assert_eq!(en_passant_pawn.get_colour(), &Colour::White);
}

#[test]
fn black_can_play_move() {
    let mut game = Game::new(1);

    let from_square = Square::new(Rank::Two, File::A);
    let to_square = Square::new(Rank::Four, File::A);
    let _ = game
        .play_unvalidated_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::H);
    let to_square = Square::new(Rank::Five, File::H);
    let _ = game
        .play_unvalidated_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Two, File::B);
    let to_square = Square::new(Rank::Four, File::B);
    let _ = game
        .play_unvalidated_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Five, File::H);
    let to_square = Square::new(Rank::Four, File::H);
    let _ = game
        .play_unvalidated_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Two, File::G);
    let captured_piece_square = Square::new(Rank::Four, File::G);
    let _ = game
        .play_unvalidated_move(&Colour::White, &from_square, &captured_piece_square)
        .unwrap();

    let from_square = Square::new(Rank::Four, File::H);
    let to_square = Square::new(Rank::Three, File::G);
    let result = game.play_unvalidated_move(&Colour::Black, &from_square, &to_square);

    assert_eq!(result, Ok(&GameStatus::ToPlayWhite));
    assert_eq!(game.get_piece_at_square(&from_square), None);
    assert_eq!(game.get_piece_at_square(&captured_piece_square), None);

    let en_passant_pawn = game.get_piece_at_square(&to_square).unwrap();
    assert_eq!(en_passant_pawn.get_piece_type(), &PieceType::Pawn);
    assert_eq!(en_passant_pawn.get_colour(), &Colour::Black);
}
