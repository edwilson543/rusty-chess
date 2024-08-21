use rocket;
use rocket::http;
use rocket::serde::json;
use rocket_ws;

use crate::config;
use crate::services::games;

use super::outbound_messages;

#[rocket::post("/games/start")]
pub async fn start_game() -> (http::Status, json::Json<String>) {
    let repo = config::get_game_repo();
    let game = games::start_game(repo);
    let payload = serde_json::to_string(&game).unwrap();
    (http::Status::Created, json::Json(payload))
}

#[rocket::get("/play")]
pub async fn play(ws: rocket_ws::WebSocket) -> rocket_ws::Stream!['static] {
    rocket_ws::Stream! { ws =>
        yield outbound_messages::new_game_message();
        for await message in ws {
            yield message?;
        }
    }
}
