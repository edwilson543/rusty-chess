use rocket;
use rocket::http;
use rocket::serde::json;
use rocket_ws;
use serde;

use crate::config;
use crate::domain::gameplay::chess_set;
use crate::services::games;

use super::{deserializers, outbound_messages};

#[rocket::post("/games/start")]
pub async fn start_game() -> (http::Status, json::Json<String>) {
    let repo = config::get_game_repo();
    let game = games::start_game(repo);
    let payload = serde_json::to_string(&game).unwrap();
    (http::Status::Created, json::Json(payload))
}

#[rocket::post("/games/<id>/play-move", data = "<play_move>")]
pub async fn play_move(
    id: i32,
    play_move: json::Json<deserializers::Move<'_>>,
) -> (http::Status, json::Json<String>) {
    let repo = config::get_game_repo();

    match games::play_move(
        repo,
        &id,
        &play_move.get_player(),
        &play_move.get_from_square(),
        &play_move.get_to_square(),
    ) {
        Ok(game) => {
            let payload = serde_json::to_string(&game).unwrap();
            (http::Status::Ok, json::Json(payload))
        }
        Err(err) => {
            let payload = json::json!({"error": format!("{}", err)});
            (
                http::Status::BadRequest,
                json::Json(json::to_string(&payload).unwrap()),
            )
        }
    }
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
