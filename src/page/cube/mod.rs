use maud::{
    html,
    Markup,
};

use crate::asset;

pub(crate) fn create(asset: &str, faces: [Markup; 6]) -> Markup {
    crate::page::create(
        html! {
            link rel="stylesheet" type="text/css" href=(asset!("cube.css"));
            link rel="stylesheet" type="text/css" href=(asset);
        },
        html! {
            div class="scene" {
                div class="cube" {
                    @for (name, content) in ["front", "top", "back", "bottom", "right", "left"].iter().zip(faces) {
                        div class=(format!("face {name}")) {
                            (content)
                        }
                    }
                }
            }

            script src=(asset!("cube.js"));
        },
    )
}
