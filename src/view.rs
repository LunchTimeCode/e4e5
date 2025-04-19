use maud::{Markup, html};
use rocket::{Route, response::content};

use crate::chess_view;


pub fn page(markup: Markup) -> Markup {
    html! {
       html  data-theme="dim" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content="Jobs";
                ({frontend::resources()})
                ({title("e4e5")})
            }

            body {
                (markup)
        }
       }
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}

pub mod frontend {
    use maud::{Markup, PreEscaped, html};

    const DAISY: &str = r#"<link  href="/_assets/daisy.css" rel="stylesheet" type="text/css">"#;
    const TAIL: &str = r#"<script src="/_assets/tail.js"></script>"#;
    const CHESS: &str = r#"<script type="module" src="/_assets/chess.js"></script>"#;
    const CHESS_LOG: &str = r#"<script type="module" src="/_assets/chess_log.js"></script>"#;
    const CHESS_FUN: &str = r#"<script type="module" src="/_assets/chess_fun.js"></script>"#;
    const DAISY_THEMES: &str =
        r#"<link  href="/_assets/themes.css" rel="stylesheet" type="text/css">"#;

    pub fn resources() -> Markup {
        html! {
        (PreEscaped(TAIL))
        (PreEscaped(CHESS))
        (PreEscaped(CHESS_LOG))
        (PreEscaped(CHESS_FUN))
        (PreEscaped(DAISY))
        (PreEscaped(DAISY_THEMES))
        }
    }
}

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page(body_m()).into_string())
}

fn body_m() -> Markup {
    let navbar = fighting_daisy::components::navbar::Navbar::new("e4e5")
        .add_item(html! {
            button .(fighting_daisy::daisy::btn::primary()) { "Test" }
        })
        .render();

    html! {
    body {
    header {
        (navbar)
    }
    
    div .box .w-96 .h-96 {
            (chess_view::chess_board())
    }
    


        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body])
}
