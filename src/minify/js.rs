use minify_js::{
    Session,
    TopLevelMode,
};

pub fn js<S: AsRef<str>>(input: S) -> String {
    let session = Session::new();
    let mut out = Vec::new();

    minify_js::minify(
        &session,
        TopLevelMode::Module,
        input.as_ref().as_bytes(),
        &mut out,
    )
    .unwrap();

    String::from_utf8(out).unwrap()
}
