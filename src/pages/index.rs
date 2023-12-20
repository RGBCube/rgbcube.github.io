use maud::{
    html,
    Markup,
};

use crate::cube;

pub async fn generate() -> Markup {
    cube::create(
        STYLING,
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

const STYLING: &str = r#"
.frame:hover {
    background-color: #FFFF00;
}

.face::after {
    z-index: -1;

    content: "";

    height: inherit;
    width: inherit;

    position: absolute;
}

.front {
    background: linear-gradient(to bottom, cyan, blue);
}

.front::after {
    background: linear-gradient(to bottom, white, magenta);
    mask-image: linear-gradient(to left, magenta, transparent);
}

.top {
    background: linear-gradient(to bottom, lime, cyan);
}

.top::after {
    background: linear-gradient(to bottom, yellow, white);
    mask-image: linear-gradient(to left, white, transparent);
}

.back {
    background: linear-gradient(to bottom, yellow, red);
}

.back::after {
    background: linear-gradient(to bottom, lime, black);
    mask-image: linear-gradient(to left, black, transparent);
}

.bottom {
    background: linear-gradient(to bottom, blue, black);
}

.bottom::after {
    background: linear-gradient(to bottom, magenta, red);
    mask-image: linear-gradient(to left, red, transparent);
}

.right {
    background: linear-gradient(to bottom, white, magenta);
}

.right::after {
    background: linear-gradient(to bottom, yellow, red);
    mask-image: linear-gradient(to left, red, transparent);
}

.left {
    background: linear-gradient(to bottom, lime, black);
}

.left::after {
    background: linear-gradient(to bottom, cyan, blue);
    mask-image: linear-gradient(to left, blue, transparent);
}
"#;
