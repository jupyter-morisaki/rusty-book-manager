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
use crate::handler::checkout::checkout_book;
use crate::handler::checkout::checkout_history;
use crate::handler::checkout::return_book;
use crate::handler::checkout::show_checked_out_list;

pub fn build_book_router() -> Router<AppRegistry> {
    let book_router = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book))
        .route("/:book_id", put(update_book))
        .route("/:book_id", delete(delete_book));

    let checkout_router = Router::new()
        .route("/checkouts", get(show_checked_out_list))
        .route("/:book_id/checkouts", post(checkout_book))
        .route(
            "/:book_id/checkouts/:checkout_id/returned",
            put(return_book),
        )
        .route("/:book_id/checkout-history", get(checkout_history));

    let router = book_router.merge(checkout_router);

    Router::new().nest("/books", router)
}
