mod constants;
mod cube;
mod minify;
mod page;
mod pages;

use anyhow::Context;
use axum::{
    routing::get,
    Router,
};
use constants::*;
use env_logger::Target;
use log::LevelFilter;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    let app = Router::new()
        .route("/", get(pages::index))
        .fallback(pages::_404);

    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .with_context(|| "Failed to bind to 0.0.0.0:80")?;

    axum::serve(listener, app)
        .await
        .with_context(|| "Server crashed")?;

    Ok(())
}
