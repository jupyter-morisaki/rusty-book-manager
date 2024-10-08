use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use garde::Validate;

use kernel::model::book::event::DeleteBook;
use kernel::model::id::BookId;
use registry::AppRegistry;
use shared::error::AppError;
use shared::error::AppResult;

use crate::extractor::AuthorizedUser;
use crate::model::book::BookListQuery;
use crate::model::book::BookResponse;
use crate::model::book::CreateBookRequest;
use crate::model::book::PaginatedBookResponse;
use crate::model::book::UpdateBookRequest;
use crate::model::book::UpdateBookRequestWithIds;

pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate()?;

    registry
        .book_repository()
        .create(req.into(), user.id())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    _user: AuthorizedUser,
    Query(query): Query<BookListQuery>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<PaginatedBookResponse>> {
    query.validate()?;

    registry
        .book_repository()
        .find_all(query.into())
        .await
        .map(PaginatedBookResponse::from)
        .map(Json)
}

pub async fn show_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound(
                "The specific book was not found".into(),
            )),
        })
}

pub async fn update_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate()?;

    let update_book = UpdateBookRequestWithIds::new(book_id, user.id(), req);

    registry
        .book_repository()
        .update(update_book.into())
        .await
        .map(|_| StatusCode::OK)
}

pub async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let delete_book = DeleteBook {
        book_id,
        requested_user: user.id(),
    };
    registry
        .book_repository()
        .delete(delete_book)
        .await
        .map(|_| StatusCode::OK)
}
