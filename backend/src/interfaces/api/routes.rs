use rocket;
use rocket_ws;

use super::outbound_messages;

#[rocket::get("/play")]
pub async fn play(ws: rocket_ws::WebSocket) -> rocket_ws::Stream!['static] {
    rocket_ws::Stream! { ws =>
        yield outbound_messages::new_game_message();
        for await message in ws {
            yield message?;
        }
    }
}
