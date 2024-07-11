use rocket;
use rocket::http;
use rocket::serde::json;

use crate::config;
use crate::services::games;

#[rocket::post("/games")]
pub async fn start_game() -> (http::Status, json::Json<String>) {
    // TODO -> change to using a websocket

    let uow = config::get_unit_of_work();
    let game = games::start_game(uow);
    let payload = json::to_string(&game).unwrap();
    (http::Status::Created, json::Json(payload))
}
