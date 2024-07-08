use chess::interfaces::api;
use rocket;

#[rocket::launch]
fn launch() -> rocket::Rocket<rocket::Build> {
    api::rocket_build()
}
