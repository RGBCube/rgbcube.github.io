use maud::{
    html,
    Markup,
    PreEscaped,
    DOCTYPE,
};

fn property(name: &str, content: &str) -> Markup {
    PreEscaped(
        html! {
            meta property=(name) content=(content);
        }
        .into_string(),
    )
}

fn pname(name: &str, content: &str) -> Markup {
    PreEscaped(
        html! {
            meta name=(name) content=(content);
        }
        .into_string(),
    )
}

pub fn create(head: Markup, body: Markup) -> Markup {
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

            @let description = "The official website and link portal of RGBCube and his work.";
            (pname("description", description))
            (property("og:description", description))

            link rel="icon" href="/assets/icon.gif" type="image/gif";

            (property("og:image", "thumbnail.png"))
            (property("og:image:type", "image/png"))
            (property("og:image:height", "1080"))
            (property("og:image:width", "600"))

            (property("og:url", crate::URL))
            link rel="canonical" href=(crate::URL);

            (PreEscaped(head.into_string()))
        }

        body {
            (PreEscaped(body.into_string()))
        }
    }
}
