#![feature(lazy_cell)]

mod constants;
mod cube;
mod minify;
mod page;
mod routes;

use constants::*;
use env_logger::Target;
use log::LevelFilter;
use routes::*;
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let routes = index::filter().or(assets::filter()).or(_404::filter());

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}
