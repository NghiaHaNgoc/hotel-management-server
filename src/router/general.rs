use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::service::general;

pub fn general_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/user/profile", get(general::get_profile))
        .route("/user/profile", post(general::update_profile))
        // .route("/user/change-password", post(general::change_password))
        .with_state(db)
}
