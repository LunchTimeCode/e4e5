use core::fmt;

use maud::{Markup, html};

#[allow(unused)]
enum Position {
    Start,
    None,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Start => write!(f, "start"),
            Position::None => write!(f, "none"),
        }
    }
}

impl Position {
    fn new() -> Self {
        Position::Start
    }
}

pub fn chess_board() -> Markup {
    let pos = Position::new().to_string();

    html! {
        chess-board showNotation="true" position=(pos) draggable-pieces {
        }

    }
}
    
    
pub mod game_history {
    use maud::{html, Markup};

    use crate::{chess_view::game_move, game::ChessGame};
    
    pub fn game_history(game: &ChessGame) -> Markup {   
    
      html! {
            div {
                h2 { "Game History" }
                ul {
                   @for mo_ve in game.moves().iter()  {
                        li { (game_move::render(mo_ve)) }
                    }
                }
            }
        }
    }
    
}

mod game_move{
    use maud::{html, Markup};

    use crate::chess::ChessMove;
    
    pub fn render(mo_ve: &ChessMove) -> Markup { 
       let color = match mo_ve.color.as_str() {
           "w" => "bg-white-100",
           "b" => "bg-black-100",
           _ => "unknown",
       };
       html!{
           li {
                   div class="card w-96 shadow-sm" .(color) {
                       figure {
                           img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp" alt="Shoes";
                       }
                       div class="card-body" {
                           h2 class="card-title" {
                             (mo_ve.piece)  (mo_ve.from) "to" (mo_ve.to)
                           }
                           p {
                               (mo_ve.before)
                               (mo_ve.after)
                           }
                           div class="card-actions justify-end" {
                               button class="btn btn-primary" {
                                   "Next"
                               }
                           }
                       }
                   }
               }
        
           
       }    
    }
    
}
