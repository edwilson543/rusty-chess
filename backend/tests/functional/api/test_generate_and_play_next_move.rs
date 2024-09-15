use chess::interfaces::api;
use rocket::http;
use rocket::local;
use serde_json;

use chess::config;
use chess::domain::game;

#[test]
fn can_generate_and_play_opening_moves_for_white_then_black() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let mut repo = config::get_game_repo();
    let game = repo.create();

    let url = format!("/api/games/{}/generate-and-play-next-move/", game.get_id());
    let payload = serde_json::json!(
        {"engine": "Random"}
    );

    // Automate white's opening move.
    let request = client.post(&url).body(payload.to_string());
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::Ok);
    assert_eq!(response.content_type(), Some(http::ContentType::JSON));

    let game = repo.get(game.get_id()).unwrap();
    assert_eq!(game.get_status(), &game::GameStatus::ToPlayBlack);
    assert_eq!(game.get_chessboard_history().len(), 2);

    // Automate black's opening move.
    let request = client.post(&url).body(payload.to_string());
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::Ok);
    assert_eq!(response.content_type(), Some(http::ContentType::JSON));

    let game = repo.get(game.get_id()).unwrap();
    assert_eq!(game.get_status(), &game::GameStatus::ToPlayWhite);
    assert_eq!(game.get_chessboard_history().len(), 3);
}

#[test]
fn bad_response_when_game_does_not_exist() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let url = format!("/api/games/{}/generate-and-play-next-move/", 12345);
    let payload = serde_json::json!(
        {"engine": "Random"}
    );

    let request = client.post(url).body(payload.to_string());
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::BadRequest);
    assert_eq!(response.content_type(), Some(http::ContentType::JSON));

    assert_eq!(
        response.into_string().unwrap(),
        "\"{\\\"error\\\":\\\"Game 12345 does not exist\\\"}\""
    );
}
