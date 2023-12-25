use std::sync::LazyLock;

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
});

pub fn filter() -> impl Filter {
    warp::any().map(|| &*PAGE)
}
