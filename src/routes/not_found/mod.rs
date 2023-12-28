use std::sync::LazyLock;

use maud::{
    html,
    Markup,
};

use crate::{
    asset,
    page::cube,
};

static PAGE: LazyLock<Markup> = LazyLock::new(|| {
    let face = html! {
       div class="frame" { "404" }
       div class="square black" {}
       div class="square magenta" {}
       div class="square magenta" {}
       div class="square black" {}
    };

    cube::create(
        &asset!("404.css"),
        [
            face.clone(),
            face.clone(),
            face.clone(),
            face.clone(),
            face.clone(),
            face.clone(),
        ],
    )
});

pub fn init() {
    let _ = &*PAGE;
}

pub async fn handler() -> &'static Markup {
    &*PAGE
}
