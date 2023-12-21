use std::array;

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
        minify::css(embed::string!("404.css")),
        array::from_fn(|_| {
            (html! {
               div class="frame" { "404" }
               div class="square black" {}
               div class="square magenta" {}
               div class="square magenta" {}
               div class="square black" {}
            })
            .clone()
        }),
    )
}
