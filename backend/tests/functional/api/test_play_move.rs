use chess::interfaces::api;
use rocket::http;
use rocket::local;
use serde_json;

use chess::config;

#[test]
fn can_play_an_opening_move() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let mut repo = config::get_game_repo();
    let game = repo.create();

    let url = format!("/api/games/{}/play-move/", game.get_id());
    let payload = serde_json::json!(
        {"player": "White", "from_square": "E2", "to_square": "E4"}
    );

    let request = client.post(url).body(payload.to_string());
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::Ok);
    assert_eq!(response.content_type(), Some(http::ContentType::JSON));

    // TODO -> check the JSON.
}

#[test]
fn bad_response_when_opening_move_is_invalid() {}
