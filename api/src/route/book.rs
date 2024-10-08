use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;

use registry::AppRegistry;

use crate::handler::book::delete_book;
use crate::handler::book::register_book;
use crate::handler::book::show_book;
use crate::handler::book::show_book_list;
use crate::handler::book::update_book;

pub fn build_book_router() -> Router<AppRegistry> {
    let router = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book))
        .route("/:book_id", put(update_book))
        .route("/:book_id", delete(delete_book));

    Router::new().nest("/books", router)
}
