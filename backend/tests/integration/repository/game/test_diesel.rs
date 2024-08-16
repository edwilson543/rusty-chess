#[cfg(test)]
mod tests {
    use chess::domain::gameplay::chess_set::{Colour, File, Rank, Square};
    use chess::domain::gameplay::game;
    use chess::repository::{DieselGameRepository, GameRepository};

    #[test]
    fn can_create_then_get_then_update_game() {
        let mut repo = DieselGameRepository::new();
        let created_game = repo.create();

        let maybe_game = repo.get(created_game.get_id());
        let mut got_game = maybe_game.unwrap();

        let expected_new_game = game::Game::new(*created_game.get_id());
        assert_eq!(got_game, expected_new_game);

        // Play an opening move.
        let from_square = Square::new(Rank::Two, File::E);
        let to_square = Square::new(Rank::Four, File::E);
        got_game
            .play_ordinary_move(&Colour::White, &from_square, &to_square)
            .unwrap();

        repo.update(&got_game);

        let maybe_update_game = repo.get(created_game.get_id());
        let updated_game = maybe_update_game.unwrap();
        assert_eq!(updated_game.get_status(), &game::GameStatus::ToPlayBlack);
        assert_eq!(updated_game.get_chessboard_history().len(), 2);
        assert_eq!(updated_game, got_game);
    }

    #[test]
    fn gets_none_when_game_does_not_exist() {
        let mut repo = DieselGameRepository::new();

        let maybe_game = repo.get(&123);

        assert_eq!(maybe_game, None);
    }
}
