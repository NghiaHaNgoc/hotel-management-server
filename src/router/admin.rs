use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use postgrest::Postgrest;

use crate::{layer, service::admin};

pub fn admin_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::admin_layer);
    Router::new()
        .route("/user/add", post(admin::add_user))
        .route("/user/list", get(admin::list_user))
        .route("/user/update/:user_id", post(admin::update_user))
        // .route("/type-room/list", get(admin::list_type_room))
        // .route("/type-room/add", post(admin::add_type_room))
        .route("/amenity/add", post(admin::add_amenity))
        .route("/amenity/list", get(admin::list_amenity))
        .route("/amenity/delete/:amenity_id", delete(admin::delete_amenity))
        .with_state(db)
        .layer(layer)
}
