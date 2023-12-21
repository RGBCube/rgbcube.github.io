use std::io::{
    Cursor,
    Read,
};

use axum::Router;
use dashmap::DashMap;
use tar::Archive;

pub fn router() -> Router {
    let app = Router::new();

    let tar_contents = embed::bytes!("../../assets.tar");
    let mut archive = Archive::new(Cursor::new(tar_contents.as_ref()));

    let archive_map: DashMap<String, Vec<u8>> = DashMap::new();

    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();

        let path = String::from_utf8(entry.path_bytes().to_vec()).unwrap();

        // Is a directory.
        if path.ends_with("/") {
            continue;
        }

        let mut contents = Vec::new();
        entry.read_to_end(&mut contents).unwrap();

        archive_map.insert(path, contents);
    }

    app
}
