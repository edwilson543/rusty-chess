use chess::interfaces::api;
use rocket::http;
use rocket::local;

#[test]
fn can_start_a_new_game() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let request = client.post("/api/games/");
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::Created);
    assert_eq!(response.content_type(), Some(http::ContentType::JSON));
    let expected_json_snip =
        "\"{\"id\":1,\"status\":{\"ToPlay\":\"White\"},\"chessboard_history\":[{\"position\"";
    let actual_json = response.into_string().unwrap().replace(r"\", "");
    assert!(actual_json.starts_with(expected_json_snip));
}
