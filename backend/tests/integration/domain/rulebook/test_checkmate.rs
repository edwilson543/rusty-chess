use chess::domain::chess_set::{Colour, File, Rank, Square};
use chess::domain::game::{Game, GameStatus};

#[test]
fn fools_mate_by_black() {
    let mut game = Game::new(1);

    let from_square = Square::new(Rank::Two, File::F);
    let to_square = Square::new(Rank::Three, File::F);
    let _ = game
        .play_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Seven, File::E);
    let to_square = Square::new(Rank::Six, File::E);
    let _ = game
        .play_ordinary_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Two, File::G);
    let to_square = Square::new(Rank::Four, File::G);
    let _ = game
        .play_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Eight, File::D);
    let to_square = Square::new(Rank::Four, File::H);
    let _ = game
        .play_ordinary_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    assert_eq!(game.get_status(), &GameStatus::WonByBlack);
}

#[test]
fn scholars_mate_by_white() {
    let mut game = Game::new(1);

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

    let from_square = Square::new(Rank::Eight, File::B);
    let to_square = Square::new(Rank::Six, File::C);
    let _ = game
        .play_ordinary_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::One, File::F);
    let to_square = Square::new(Rank::Four, File::C);
    let _ = game
        .play_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Eight, File::G);
    let to_square = Square::new(Rank::Six, File::F);
    let _ = game
        .play_ordinary_move(&Colour::Black, &from_square, &to_square)
        .unwrap();

    let from_square = Square::new(Rank::Five, File::H);
    let to_square = Square::new(Rank::Seven, File::F);
    let _ = game
        .play_ordinary_move(&Colour::White, &from_square, &to_square)
        .unwrap();

    assert_eq!(game.get_status(), &GameStatus::WonByWhite);
}
