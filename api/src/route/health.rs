use axum::routing::get;
use axum::Router;

use registry::AppRegistry;

use crate::handler::health::health_check;
use crate::handler::health::health_check_db;

pub fn build_health_check_router() -> Router<AppRegistry> {
    let router = Router::new()
        .route("/", get(health_check))
        .route("/db", get(health_check_db));

    Router::new().nest("/health", router)
}
