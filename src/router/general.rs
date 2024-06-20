use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::service::account;

pub fn general_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/user/profile", get(account::get_profile))
        .route("/user/profile", post(account::update_profile))
        .route("/user/change-password", post(account::change_password))
        .with_state(db)
}
