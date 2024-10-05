use axum::routing::get;
use axum::routing::post;
use axum::Router;

use registry::AppRegistry;

use crate::handler::book::register_book;
use crate::handler::book::show_book;
use crate::handler::book::show_book_list;

pub fn build_book_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book));

    Router::new().nest("/books", routers)
}
