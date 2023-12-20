use maud::{
    html,
    Markup,
};

use crate::page;

const FACES: [&str; 6] = ["front", "top", "back", "bottom", "right", "left"];

pub fn create(styling: &str, faces: [Markup; 6]) -> Markup {
    page::create(
        html! {
            link href="cube.css" rel="stylesheet";
        },
        html! {
            style {
                (styling)
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
        },
    )
}
