use embed_file::embed_string as embed;
use maud::{
    html,
    Markup,
};

use crate::{
    minify,
    page,
};

const FACES: [&str; 6] = ["front", "top", "back", "bottom", "right", "left"];

pub fn create<S: AsRef<str>>(styling: S, faces: [Markup; 6]) -> Markup {
    page::create(
        html! {
            style {
                (minify::css(embed!("cube.css")))
            }
        },
        html! {
            style {
                (styling.as_ref())
            }

            div class="scene" {
                div class="cube" {
                    @for (name, content) in FACES.iter().zip(faces) {
                        div class=(format!("face {name}")) {
                            (content)
                        }
                    }
                }
            }

            script {
                (minify::js(embed!("cube.js")))
            }
        },
    )
}
