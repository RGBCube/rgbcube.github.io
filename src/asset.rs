use axum::{
    body::Body,
    http::header,
    response::Response,
    routing,
};
use bytes::Bytes;
use minify_js::{
    Session,
    TopLevelMode,
};

fn route(path: &'static str, content: Bytes) -> String {
    crate::route(
        path,
        routing::get(async move || {
            Response::builder()
                .header(
                    header::CONTENT_TYPE,
                    mime_guess::from_path(path)
                        .first_or_octet_stream()
                        .to_string(),
                )
                .body(Body::from(content.clone()))
                .unwrap()
        }),
    );

    format!("/assets/{path}")
}

pub(crate) fn __register_asset(
    path: &'static str,
    min_path: &'static str,
    content: Vec<u8>,
) -> String {
    let (minifiable, minified_content) = match path.rsplit_once(".").unwrap_or_default().1 {
        "js" => {
            let mut minified = Vec::new();

            minify_js::minify(
                &Session::new(),
                TopLevelMode::Module,
                &content,
                &mut minified,
            )
            .unwrap();

            (true, minified)
        },
        _ => (false, content.clone()), // TODO: Minify CSS.
    };

    if minifiable {
        route(path, Bytes::from(content));
        route(min_path, Bytes::from(minified_content))
    }
    else {
        route(path, Bytes::from(content))
    }
}

#[macro_export]
macro_rules! asset {
    ($path:literal) => {{
        crate::asset::__register_asset($path, $path, ::embed::bytes!($path).to_vec())
    }};
}
