use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use postgrest::Postgrest;

use crate::{
    layer,
    service::{amenity, room, type_room, users},
};

pub fn admin_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::admin_layer);
    Router::new()
        // User
        .route("/user/add", post(users::add_user))
        .route("/user/list", get(users::list_user))
        .route("/user/update/:user_id", post(users::update_user))
        // Type room
        .route("/type-room/list", get(type_room::list_type_room))
        .route("/type-room/add", post(type_room::add_type_room))
        .route(
            "/type-room/update/:type_room_id",
            post(type_room::update_type_room),
        )
        .route(
            "/type-room/delete/:type_room_id",
            delete(type_room::delete_type_room),
        )
        // Room
        .route("/room/add", post(room::add_room))
        // Amenity
        .route("/amenity/add", post(amenity::add_amenity))
        .route("/amenity/list", get(amenity::list_amenity))
        .route(
            "/amenity/delete/:amenity_id",
            delete(amenity::delete_amenity),
        )
        .with_state(db)
        .layer(layer)
}
