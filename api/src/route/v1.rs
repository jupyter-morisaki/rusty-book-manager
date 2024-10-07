use axum::Router;

use registry::AppRegistry;

use crate::route::book::build_book_router;
use crate::route::health::build_health_check_router;
use crate::route::user::build_user_router;

pub fn routes() -> Router<AppRegistry> {
    let router = Router::new()
        .merge(build_health_check_router())
        .merge(build_book_router())
        .merge(build_user_router());

    Router::new().nest("/api/v1", router)
}
