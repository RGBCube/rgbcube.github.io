use anyhow::Context;
use minify_js::{
    Session,
    TopLevelMode,
};

pub fn js(input: &str) -> String {
    let session = Session::new();
    let mut out = Vec::new();

    minify_js::minify(&session, TopLevelMode::Module, input.as_bytes(), &mut out)
        .with_context(|| format!("Failed to minify::js: {code}."))
        .unwrap();

    String::from_utf8(out)
        .with_context(|| format!("Failed to create a string from minify::js output: {out:?}."))
        .unwrap()
}
