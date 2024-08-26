use chess::config;
use chess::interfaces::api;
use rocket::http;
use rocket::local;

#[test]
fn can_fetch_state_of_game_that_exists() {
    {
        let build = api::rocket_build();
        let client = local::blocking::Client::tracked(build).unwrap();

        let mut repo = config::get_game_repo();
        let game = repo.create();

        let url = format!("/api/games/{}", game.get_id());
        let request = client.get(url);
        let response = request.dispatch();

        assert_eq!(response.status(), http::Status::Ok);
        assert_eq!(response.content_type(), Some(http::ContentType::JSON));
    }
}
