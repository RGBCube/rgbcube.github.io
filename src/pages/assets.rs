use std::io::Read;

use axum::Router;
use dashmap::DashMap;
use embed_file::embed_string as embed;
use stringreader::StringReader;
use tar::Archive;

pub fn router() -> Router {
    let app = Router::new();

    let tar_contents = embed!("assets.tar");
    let mut archive = Archive::new(StringReader::new(&tar_contents));
    let archive_map: DashMap<String, Vec<u8>> = DashMap::new();

    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();

        let path = String::from_utf8(entry.path_bytes().to_vec()).unwrap();

        let mut contents = Vec::new();
        entry.read_to_end(&mut contents).unwrap();

        archive_map.insert(path, contents);
    }

    println!("{archive_map:?}");

    app
}
