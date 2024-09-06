use chess::domain::chess_set::{Colour, File, Piece, PieceType, Rank, Square};
use chess::domain::game::{Game, GameStatus};

#[test]
fn white_can_castle_kingside() {
    let mut game = Game::new(1);

    // Move the knight out the way.
    let from_square = Square::new(Rank::One, File::G);
    let to_square = Square::new(Rank::Three, File::H);
    let _ = game
        .play_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::A);
    let to_square = Square::new(Rank::Six, File::A);
    let _ = game
        .play_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    // Move the knight's pawn out the way.
    let from_square = Square::new(Rank::Two, File::G);
    let to_square = Square::new(Rank::Three, File::G);
    let _ = game
        .play_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::B);
    let to_square = Square::new(Rank::Six, File::B);
    let _ = game
        .play_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    // Move the bishop out the way.
    let from_square = Square::new(Rank::One, File::F);
    let to_square = Square::new(Rank::Two, File::G);
    let _ = game
        .play_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::C);
    let to_square = Square::new(Rank::Six, File::C);
    let _ = game
        .play_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    // Castle!!!
    let from_square = Square::new(Rank::One, File::E);
    let to_square = Square::new(Rank::One, File::G);
    let result = game.play_move(&Colour::White, &from_square, &to_square);

    assert_eq!(result, Ok(&GameStatus::ToPlayBlack));
    assert_eq!(game.get_piece_at_square(&from_square), None);
    let white_king = Piece::new(Colour::White, PieceType::King);
    assert_eq!(game.get_piece_at_square(&to_square), Some(white_king));

    let white_rook_to_square = Square::new(Rank::One, File::F);
    let white_rook = Piece::new(Colour::White, PieceType::Rook);
    assert_eq!(
        game.get_piece_at_square(&white_rook_to_square),
        Some(white_rook)
    );
}
