use rocket;
use rocket::http;
use rocket::serde::json;
use rocket_ws;

use crate::config;
use crate::services::games;

#[rocket::post("/start-game")]
pub async fn start_game() -> (http::Status, json::Json<String>) {
    // TODO -> change to using a websocket

    let uow = config::get_unit_of_work();
    let game = games::start_game(uow);
    let payload = json::to_string(&game).unwrap();
    (http::Status::Created, json::Json(payload))
}

#[rocket::get("/ws-trial")]
pub async fn ws_trial(ws: rocket_ws::WebSocket) -> rocket_ws::Stream!['static] {
    rocket_ws::Stream! { ws =>
        yield game_started_message();
        for await message in ws {
            yield message?;
        }
    }
}

fn game_started_message() -> rocket_ws::Message {
    let uow = config::get_unit_of_work();
    let game = games::start_game(uow);
    let payload = json::to_string(&game).unwrap();
    rocket_ws::Message::Text(payload)
}
