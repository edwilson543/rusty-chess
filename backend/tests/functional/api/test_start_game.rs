use chess::interfaces::api;
use rocket::http;
use rocket::local;

#[test]
fn can_start_a_new_game() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    let request = client.post("/api/start-game/");
    let response = request.dispatch();

    assert_eq!(response.status(), http::Status::Created);
    assert_eq!(response.content_type(), Some(http::ContentType::JSON));
    let expected_json_snip =
        "\"{\"id\":1,\"status\":{\"ToPlay\":\"W\"},\"chessboard_history\":[{\"position\"";
    let actual_json = response.into_string().unwrap().replace(r"\", "");
    assert!(actual_json.starts_with(expected_json_snip));
}

// #[test]
fn ws_can_start_a_new_game() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    // Create client request to initiate WebSocket opening handshake.
    let mut request = client.get("/api/ws-trial/");
    let ws_key = "SOME-KEY";

    let connection_upgrade = http::Header::new("Connection", "upgrade");
    request.add_header(connection_upgrade);
    let upgrade_to_websocket = http::Header::new("Upgrade", "websocket");
    request.add_header(upgrade_to_websocket);
    let upgrade_to_websocket = http::Header::new("Sec-WebSocket-Version", "13");
    request.add_header(upgrade_to_websocket);
    let upgrade_to_websocket = http::Header::new("Sec-WebSocket-Key", ws_key.clone());
    request.add_header(upgrade_to_websocket);

    let response = request.dispatch();

    let encoded_accept_key = "py4cEXb0rIHAdKoYJjw61ZJTHR4=";
    assert_eq!(
        response.headers().get("Sec-WebSocket-Accept").next(),
        Some(encoded_accept_key)
    );
    assert_eq!(response.status(), http::Status::SwitchingProtocols);
}
