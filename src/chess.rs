use std::str::FromStr;

use rocket::response::content::RawHtml;
use rocket::Route;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::_State;
use crate::chess_view;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChessMove {
    pub color: String,
    pub from: String,
    pub to: String,
    pub piece: String,
    pub flags: String,
    pub san: String,
    pub lan: String,
    pub before: String,
    pub after: String,
}

#[post("/<id>/move", format = "application/json", data = "<chess_move>")]
pub async fn chess_move(
    state: &_State,
    id: String,
    chess_move: rocket::serde::json::Json<ChessMove>,
) -> Option<String> {
    let mut state = state.get().await;
    let id = uuid::Uuid::from_str(&id).ok()?;
    state.get_game_or_create(id).push_move(chess_move.0);

    Some("Ok".to_string())
}

#[get("/<id>/status")]
pub async fn chess_status(state: &_State, id: String) -> Option<rocket::response::content::RawHtml<String>> {
    let mut state = state.get().await;
    let id = uuid::Uuid::from_str(&id).ok()?;
   let game = state.get_game_or_create(id);
   let m = chess_view::game_history::game_history(game);

   Some(RawHtml(m.into_string()))
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/api/chess", routes![chess_move, chess_status])
}
