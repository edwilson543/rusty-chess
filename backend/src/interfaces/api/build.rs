use super::routes;
use rocket;

pub fn rocket_build() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount(
        "/api",
        rocket::routes![
            routes::start_game,
            routes::get_game_state,
            routes::play_move,
            routes::play
        ],
    )
}
