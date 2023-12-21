use maud::{
    html,
    Markup,
};

use crate::{
    cube,
    minify,
};

pub async fn generate() -> Markup {
    cube::create(
        minify::css(embed::string!("index.css")),
        [
            html! {
              a href="contact" {
                div class="frame" {
                  "contact"
                }
              }
            },
            html! {
              a href="github" {
                div class="frame" {
                  "github"
                }
              }
            },
            html! {},
            html! {},
            html! {},
            html! {},
        ],
    )
}
