use super::routes;
use rocket;
use rocket::fs;

pub fn rocket_build() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/api",
            rocket::routes![routes::start_game, routes::ws_trial],
        )
        .mount(
            "/",
            fs::FileServer::from(fs::relative!("src/interfaces/static")),
        )
}
