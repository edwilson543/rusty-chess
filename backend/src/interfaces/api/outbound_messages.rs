use crate::config;
use crate::services::games;

use serde;
use serde_json;

#[derive(serde::Serialize)]
enum MessageName {
    NewGame,
}

#[derive(serde::Serialize)]
struct Message {
    name: MessageName,
    payload: String,
}

pub fn new_game_message() -> rocket_ws::Message {
    let uow = config::get_unit_of_work();
    let game = games::start_game(uow);
    let payload = serde_json::to_string(&game).unwrap();
    message_from_payload(MessageName::NewGame, payload)
}

fn message_from_payload(name: MessageName, payload: String) -> rocket_ws::Message {
    let message = Message {
        name: name,
        payload: payload,
    };
    let message_json = serde_json::to_string(&message).unwrap();
    rocket_ws::Message::Text(message_json)
}
