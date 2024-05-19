use std::sync::Arc;

use axum::{middleware, Router};
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;

use crate::middleware::authenticated_layer;

mod general;
mod public;

pub fn all_router(db: Arc<Postgrest>) -> Router {
    let app = Router::new()
        .merge(public::public_router(db.clone()))
        .merge(authenticated_router(db.clone()))
        .layer(CorsLayer::very_permissive());
    app
}

pub fn authenticated_router(db: Arc<Postgrest>) -> Router {
    let authenticated_layer = middleware::from_fn_with_state(db.clone(), authenticated_layer);
    let app = Router::new()
        .merge(general::general_router(db))
        .layer(authenticated_layer);
    app
}
