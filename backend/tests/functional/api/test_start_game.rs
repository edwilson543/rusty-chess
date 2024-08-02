use chess::interfaces::api;
use rocket::http;
use rocket::local;

// #[test]
#[allow(dead_code)]
fn ws_can_start_a_new_game() {
    let build = api::rocket_build();
    let client = local::blocking::Client::tracked(build).unwrap();

    // Create client request to initiate WebSocket opening handshake.
    let mut request = client.get("/api/play/");
    let ws_key = "SOME-KEY";

    let connection_upgrade = http::Header::new("Connection", "upgrade");
    request.add_header(connection_upgrade);
    let upgrade_to_websocket = http::Header::new("Upgrade", "websocket");
    request.add_header(upgrade_to_websocket);
    let upgrade_to_websocket = http::Header::new("Sec-WebSocket-Version", "13");
    request.add_header(upgrade_to_websocket);
    let upgrade_to_websocket = http::Header::new("Sec-WebSocket-Key", ws_key);
    request.add_header(upgrade_to_websocket);

    let response = request.dispatch();

    let encoded_accept_key = "py4cEXb0rIHAdKoYJjw61ZJTHR4=";
    assert_eq!(
        response.headers().get("Sec-WebSocket-Accept").next(),
        Some(encoded_accept_key)
    );
    assert_eq!(response.status(), http::Status::SwitchingProtocols);
}
