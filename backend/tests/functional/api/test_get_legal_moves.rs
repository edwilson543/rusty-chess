use chess::config;
use chess::domain::game;
use chess::interfaces::api;
use rocket::{http, local};

#[test]
fn can_get_legal_moves_for_active_game() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let mut repo = config::get_game_repo();
    let game = repo.create();

    let url = format!("/api/games/{}/get-legal-moves/", game.get_id());
    let request = client.get(&url);
    let response = request.dispatch();
    assert_eq!(
        response.into_string().unwrap(),
        "\"[{\\\"from_square\\\":\\\"B1\\\",\\\"to_square\\\":\\\"A3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"B1\\\",\\\"to_square\\\":\\\"C3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"G1\\\",\\\"to_square\\\":\\\"F3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"G1\\\",\\\"to_square\\\":\\\"H3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"A2\\\",\\\"to_square\\\":\\\"A3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"A2\\\",\\\"to_square\\\":\\\"A4\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"B2\\\",\\\"to_square\\\":\\\"B3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"B2\\\",\\\"to_square\\\":\\\"B4\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"C2\\\",\\\"to_square\\\":\\\"C3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"C2\\\",\\\"to_square\\\":\\\"C4\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"D2\\\",\\\"to_square\\\":\\\"D3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"D2\\\",\\\"to_square\\\":\\\"D4\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"E2\\\",\\\"to_square\\\":\\\"E3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"E2\\\",\\\"to_square\\\":\\\"E4\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"F2\\\",\\\"to_square\\\":\\\"F3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"F2\\\",\\\"to_square\\\":\\\"F4\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"G2\\\",\\\"to_square\\\":\\\"G3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"G2\\\",\\\"to_square\\\":\\\"G4\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"H2\\\",\\\"to_square\\\":\\\"H3\\\",\\\"player\\\":\\\"White\\\"},{\\\"from_square\\\":\\\"H2\\\",\\\"to_square\\\":\\\"H4\\\",\\\"player\\\":\\\"White\\\"}]\""
    );
}

#[test]
fn not_found_response_when_game_does_not_exist() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let url = format!("/api/games/{}/get-legal-moves/", 12345);
    let request = client.post(url);
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::NotFound);
}

#[test]
fn bad_request_when_game_is_complete() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let mut repo = config::get_game_repo();
    let new_game = repo.create();

    let updated_game = game::Game::reincarnate(
        new_game.get_id().clone(),
        game::GameStatus::Drawn, // Artificially end the game.
        new_game.get_chessboard_history().clone(),
    );
    repo.update(&updated_game);

    let url = format!("/api/games/{}/get-legal-moves/", updated_game.get_id());
    let request = client.get(url);
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::BadRequest);
}
