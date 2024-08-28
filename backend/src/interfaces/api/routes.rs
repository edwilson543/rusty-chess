use rocket;
use rocket::http;
use rocket::serde::json;

use crate::config;
use crate::services::games;

use super::deserializers;

#[rocket::post("/games/start")]
pub async fn start_game() -> (http::Status, json::Json<String>) {
    let repo = config::get_game_repo();
    let game = games::start_game(repo);
    let payload = serde_json::to_string(&game).unwrap();
    (http::Status::Created, json::Json(payload))
}

#[rocket::get("/games/<id>")]
pub async fn get_game_state(id: i32) -> (http::Status, json::Json<String>) {
    let mut repo = config::get_game_repo();
    match repo.get(&id) {
        Some(game) => {
            let payload = serde_json::to_string(&game).unwrap();
            (http::Status::Ok, json::Json(payload))
        }
        None => (http::Status::NotFound, json::Json("".into())),
    }
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
