use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::service::general;

pub fn general_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/profile", get(general::get_profile))
        .route("/profile", post(general::update_profile))
        // .route("/change-password", post(general::change_password))
        .with_state(db)
}
