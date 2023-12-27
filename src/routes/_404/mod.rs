use std::{
    array,
    convert::Infallible,
    sync::LazyLock,
};

use maud::html;
use warp::{
    reply::{
        self,
        Html,
    },
    Filter,
};

use crate::{
    cube,
    minify,
};

static PAGE: LazyLock<String> = LazyLock::new(|| {
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
    .into_string()
});

pub fn filter() -> impl Filter<Extract = (Html<&'static str>,), Error = Infallible> + Clone {
    warp::any().map(|| reply::html(PAGE.as_str()))
}
