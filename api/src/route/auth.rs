use axum::routing::post;
use axum::Router;

use registry::AppRegistry;

use crate::handler::auth::login;
use crate::handler::auth::logout;

pub fn routes() -> Router<AppRegistry> {
    let router = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));

    Router::new().nest("/auth", router)
}
