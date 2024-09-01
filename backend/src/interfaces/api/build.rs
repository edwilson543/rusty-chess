use super::routes;
use rocket;
use rocket_cors;

pub fn rocket_build() -> rocket::Rocket<rocket::Build> {
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::all())
        .allowed_methods(
            vec![rocket::http::Method::Get, rocket::http::Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .mount(
            "/api",
            rocket::routes![
                routes::start_game,
                routes::get_game_state,
                routes::play_move,
                routes::generate_and_play_next_move,
            ],
        )
        .attach(cors.to_cors().unwrap())
}
