use chess::interfaces::api;
use rocket::http;
use rocket::local;

#[test]
fn can_start_a_new_game() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let request = client.post("/api/games/start/");
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::Created);
    assert_eq!(response.content_type(), Some(http::ContentType::JSON));

    // TODO -> check the JSON.
}
