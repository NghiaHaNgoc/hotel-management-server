use std::sync::Arc;

use axum::{routing::post, Router};
use postgrest::Postgrest;

use crate::service::general;


pub fn public_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/login", post(general::login))
        .with_state(db)
}
