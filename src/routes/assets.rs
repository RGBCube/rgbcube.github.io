use std::{
    collections::HashMap,
    io::{
        Cursor,
        Read,
    },
    sync::LazyLock,
};

use tar::Archive;
use warp::{
    filters::fs::File,
    reject::Rejection,
    Filter,
};

static _ASSETS: LazyLock<HashMap<String, Vec<u8>>> = LazyLock::new(|| {
    let contents = embed::bytes!("../../assets.tar");

    let mut archive = Archive::new(Cursor::new(contents.as_ref()));
    let mut assets = HashMap::new();

    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();

        let path = String::from_utf8(entry.path_bytes().to_vec()).unwrap();

        // Is a directory.
        if path.ends_with("/") {
            continue;
        }

        let mut contents = Vec::new();
        entry.read_to_end(&mut contents).unwrap();

        assets.insert(path, contents);
    }

    assets
});

pub fn filter() -> impl Filter<Extract = (File,), Error = Rejection> + Clone {
    warp::path("assets").and(warp::fs::dir("assets"))
}
