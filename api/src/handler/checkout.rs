use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;

use kernel::model::checkout::event::CreateCheckout;
use kernel::model::checkout::event::UpdateReturned;
use kernel::model::id::BookId;
use kernel::model::id::CheckoutId;
use registry::AppRegistry;
use shared::error::AppResult;

use crate::extractor::AuthorizedUser;
use crate::model::checkout::CheckoutsResponse;

pub async fn checkout_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let create_checkout = CreateCheckout::new(book_id, user.id(), Utc::now());
    registry
        .checkout_repository()
        .create(create_checkout)
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn return_book(
    user: AuthorizedUser,
    Path((book_id, checkout_id)): Path<(BookId, CheckoutId)>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let update_returned = UpdateReturned::new(checkout_id, book_id, user.id(), Utc::now());
    registry
        .checkout_repository()
        .update_returned(update_returned)
        .await
        .map(|_| StatusCode::OK)
}

pub async fn show_checked_out_list(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_unreturned_all()
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}

pub async fn checkout_history(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_history_by_book_id(book_id)
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
