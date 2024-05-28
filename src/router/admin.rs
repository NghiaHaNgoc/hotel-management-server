use std::sync::Arc;

use axum::{middleware, routing::post, Router};
use postgrest::Postgrest;

use crate::{
    layer,
    service::{admin, general},
};

pub fn admin_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::admin_layer);
    Router::new()
        .route("/add-employee", post(admin::add_employee))
        .with_state(db)
        .layer(layer)
}
