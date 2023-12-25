use std::{
    array,
    sync::LazyLock,
};

use maud::{
    html,
    Markup,
};
use warp::Filter;

use crate::{
    cube,
    minify,
};

static PAGE: LazyLock<Markup> = LazyLock::new(|| {
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
});

pub fn filter() -> impl Filter {
    warp::any().map(|| &*PAGE)
}
