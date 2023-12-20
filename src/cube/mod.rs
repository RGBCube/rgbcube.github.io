use embed_file::embed_string as embed;
use maud::{
    html,
    Markup,
    PreEscaped,
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
                (PreEscaped(minify::css(embed!("cube.css"))))
            }
        },
        html! {
            style {
                (PreEscaped(styling.as_ref()))
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
                (PreEscaped(minify::js(embed!("cube.js"))))
            }
        },
    )
}
