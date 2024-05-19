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
        // .route("/profile", post(general::update_profile))
        .with_state(db)
}
