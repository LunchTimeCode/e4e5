use maud::{Markup, html};
use rocket::{Route, response::content};

use crate::chess_view;

pub fn page(markup: Markup) -> Markup {
    html! {
       html data-theme="dim" {
            head { ({index::meta()}) title { "e4e5" } }
            body { (markup) }
       }
    }
}


pub mod index {
    use maud::{Markup, PreEscaped, html};

    const DAISY: &str = r#"<link  href="/_assets/daisy.css" rel="stylesheet" type="text/css">"#;
    const FAVICON: &str = r#"<link rel="icon" type="image/x-icon" href="/_assets/favicon.ico">"#;
    const TAIL: &str = r#"<script src="/_assets/tail.js"></script>"#;
    const CHESS: &str = r#"<script type="module" src="/_assets/chess.js"></script>"#;
    const CHESS_LOG: &str = r#"<script type="module" src="/_assets/chess_log.js"></script>"#;
    const CHESS_FUN: &str = r#"<script type="module" src="/_assets/chess_fun.js"></script>"#;
    const DAISY_THEMES: &str =r#"<link  href="/_assets/themes.css" rel="stylesheet" type="text/css">"#;

    pub fn meta() -> Markup {
        html! {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta name="description" content="e4e5";
            
        (PreEscaped(TAIL))
        (PreEscaped(CHESS))
        (PreEscaped(CHESS_LOG))
        (PreEscaped(CHESS_FUN))
        (PreEscaped(DAISY))
        (PreEscaped(DAISY_THEMES))
        (PreEscaped(FAVICON))
        }
    }
}


fn body() -> Markup {
    let test_button = html! {
        button .(fighting_daisy::daisy::btn::primary()) { "Test" }
    };
    
    let navbar = fighting_daisy::components::navbar::Navbar::new("e4e5")
        .add_item(test_button)
        .render();

    html! {
    body {
     header { (navbar) }

     div .box .w-96 .h-96 .mx-auto { (chess_view::chess_board()) }
    

     }
    }
}


#[get("/")]
pub fn index_page() -> content::RawHtml<String> {
    content::RawHtml(page(body()).into_string())
}

#[get("/<id>")]
pub fn game_page(id: String) -> content::RawHtml<String> {
    content::RawHtml(page(body()).into_string())
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![index_page])
}
