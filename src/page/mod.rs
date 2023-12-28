pub mod cube;

use std::sync::LazyLock;

use cargo_toml::Manifest;
use maud::{
    html,
    Markup,
    DOCTYPE,
};

use crate::asset;

static MANIFEST: LazyLock<Manifest> =
    LazyLock::new(|| Manifest::from_str(&embed::string!("../../Cargo.toml")).unwrap());

fn property(name: &str, content: &str) -> Markup {
    html! {
        meta property=(name) content=(content);
    }
}

fn pname(name: &str, content: &str) -> Markup {
    html! {
        meta name=(name) content=(content);
    }
}

pub(crate) fn create(head: Markup, body: Markup) -> Markup {
    html! {
        (DOCTYPE)

        head {
            meta charset="UTF-8";

            (pname("viewport", "width=device-width, initial-scale=1.0"))
            (property("og:type", "website"))

            @let name = "RGBCube";

            title { (name) }
            (pname("author", name))

            (property("og:site_name", name))
            (property("og:title", name))

            @let description = MANIFEST.package.as_ref().unwrap().description().unwrap();
            (pname("description", description))
            (property("og:description", description))

            link rel="icon" href=(asset!("icon.gif")) type="image/gif";

            (property("og:image", "thumbnail.png"))
            (property("og:image:type", "image/png"))
            (property("og:image:height", "1080"))
            (property("og:image:width", "600"))

            @let url = MANIFEST.package.as_ref().unwrap().homepage().unwrap();
            (property("og:url", url))
            link rel="canonical" href=(url);

            (head)
        }

        body {
            (body)
        }
    }
}
