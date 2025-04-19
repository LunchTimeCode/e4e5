use core::fmt;

use maud::{html, Markup};


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
