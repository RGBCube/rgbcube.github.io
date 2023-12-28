#![feature(lazy_cell, async_closure)]

mod asset;
mod page;
mod routes;

use std::sync::LazyLock;

use axum::{
    routing::{
        self,
        MethodRouter,
    },
    Router,
};
use tokio::{
    net::TcpListener,
    sync::Mutex,
};

pub(crate) static ROUTER: LazyLock<Mutex<Router>> = LazyLock::new(|| Mutex::new(Router::new()));

pub(crate) fn route(path: &str, handler: MethodRouter<()>) {
    let mut router = ROUTER.blocking_lock();

    *router = router.clone().route(path, handler);

    log::info!("Registered route {path}.")
}

fn not_found_handler(handler: MethodRouter<()>) {
    let mut router = ROUTER.blocking_lock();

    *router = router.clone().fallback(handler);

    log::info!("Registered 404 handler.")
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    route("/", routing::get(routes::index::handler));

    // not_found_handler({
    //     routes::not_found::init();
    //     routes::not_found::handler.into()
    // });

    let listener = TcpListener::bind("0.0.0.0:80").await.unwrap();

    let app = ROUTER.lock().await;

    axum::serve(listener, app.clone()).await.unwrap();
}
