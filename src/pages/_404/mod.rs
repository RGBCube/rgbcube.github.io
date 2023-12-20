use std::array;

use embed_file::embed_string as embed;
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
        minify::css(embed!("404.css")),
        array::from_fn(|_| {
            (html! {
               div class="frame" {}
               div class="square black" {}
               div class="square magenta" {}
               div class="square magenta" {}
               div class="square black" {}
            })
            .clone()
        }),
    )
}
