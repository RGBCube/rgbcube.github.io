use std::sync::LazyLock;

use maud::html;
use warp::{
    reject::Rejection,
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
        minify::css(embed::string!("index.css")),
        [
            html! {
              a href="/contact" {
                div class="frame" {
                  "contact"
                }
              }
            },
            html! {
              a href="/github" {
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
    .into_string()
});

pub fn filter() -> impl Filter<Extract = (Html<&'static str>,), Error = Rejection> + Clone {
    warp::path!().map(|| reply::html(PAGE.as_str()))
}
