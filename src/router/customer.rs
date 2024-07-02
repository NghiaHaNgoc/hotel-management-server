use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::{
    layer,
    service::{reservation, room},
};

pub fn customer_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::customer_layer);
    Router::new()
        // Room
        .route("/room/list", get(room::list_room))
        .route("/room/detail/:room_id", get(room::room_detail_of_customer))
        // Reservations
        .route(
            "/reservation/add",
            post(reservation::customer::add_reservation),
        )
        .route(
            "/reservation/list",
            get(reservation::customer::list_reservation),
        )
        .with_state(db)
        .layer(layer)
}
