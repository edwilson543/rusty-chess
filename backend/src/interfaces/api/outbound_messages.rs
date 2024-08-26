use crate::config;
use crate::services::games;

use crate::domain::game;
use serde;
use serde_json;

#[derive(serde::Serialize)]
enum MessageName {
    NewGame,
}

#[derive(serde::Serialize)]
struct Message {
    name: MessageName,
    payload: game::Game,
}

impl Message {
    fn new(name: MessageName, payload: game::Game) -> Self {
        Self {
            name: name,
            payload,
        }
    }

    fn to_rocket_message(&self) -> rocket_ws::Message {
        let message_json = serde_json::to_string(&self).unwrap();
        rocket_ws::Message::Text(message_json)
    }
}

pub fn new_game_message() -> rocket_ws::Message {
    let game_repo = config::get_game_repo();
    let game = games::start_game(game_repo);
    let message = Message::new(MessageName::NewGame, game);
    message.to_rocket_message()
}
