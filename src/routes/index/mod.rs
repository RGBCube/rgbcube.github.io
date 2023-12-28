use std::sync::LazyLock;

use maud::{
    html,
    Markup,
};

use crate::{
    asset,
    page::cube,
};

static PAGE: LazyLock<BoxedFuture<Markup>>> = LazyLock::new(|| {
    cube::create(
        &asset!("index.css"),
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
});

pub fn init() {
    let _ = &*PAGE;
}

pub async fn handler() -> &'static str {
    // &*PAGE
    "asd"
}
