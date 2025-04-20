use rocket::{Build, Rocket, State};

#[macro_use]
extern crate rocket;

mod assets;
mod chess;
mod chess_view;
mod game;
mod state;
mod view;

pub type _State = State<state::shadow::_GameState>;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let (assets_path, asset_routes) = assets::api();
    let (body_path, body_routes) = view::api();
    let (chess_path, chess_routes) = chess::api();
    let game_state = state::initial();
    rocket
        .mount(assets_path, asset_routes)
        .mount(chess_path, chess_routes)
        .mount(body_path, body_routes)
        .manage(game_state)
}
