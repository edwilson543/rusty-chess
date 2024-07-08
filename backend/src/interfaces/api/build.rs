use super::routes;
use rocket;

pub fn rocket_build() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/api", rocket::routes![routes::start_game])
}
