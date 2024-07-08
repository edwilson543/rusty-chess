use rocket;
use rocket::http;
use rocket::serde::json;

#[rocket::post("/games")]
pub async fn start_game() -> (http::Status, json::Json<&'static str>) {
    // TODO -> change to using a websocket.
    (http::Status::Created, json::Json::from("New game."))
}
