use warp::Filter;

mod _404;
mod assets;
mod index;

pub fn filter() -> impl Filter {
    warp::get().and(index::filter().or(assets::filter()).or(_404::filter()))
}
