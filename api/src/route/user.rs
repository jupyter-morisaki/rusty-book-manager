use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;

use registry::AppRegistry;

use crate::handler::user::change_password;
use crate::handler::user::change_role;
use crate::handler::user::delete_user;
use crate::handler::user::get_checkouts;
use crate::handler::user::get_current_user;
use crate::handler::user::list_users;
use crate::handler::user::register_user;

pub fn build_user_router() -> Router<AppRegistry> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/password", put(change_password))
        .route("/users", get(list_users))
        .route("/users", post(register_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/users/:user_id", put(change_role))
        .route("/users/me/checkouts", get(get_checkouts))
}
