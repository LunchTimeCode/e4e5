use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod view;
mod chess_view;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let (assets_path, asset_routes) = assets::api();
    let (body_path, body_routes) = view::api();
    rocket
        .mount(assets_path, asset_routes)
        .mount(body_path, body_routes)
}