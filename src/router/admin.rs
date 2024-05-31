use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::{layer, service::admin};

pub fn admin_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::admin_layer);
    Router::new()
        .route("/user/add", post(admin::add_user))
        .route("/user/list", get(admin::list_user))
        .with_state(db)
        .layer(layer)
}
