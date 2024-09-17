use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::service::{account, amenity, type_room};

pub fn public_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/employee/sign-in", post(account::employee_sign_in))
        .route("/customer/sign-in", post(account::customer_sign_in))
        .route("/customer/sign-up", post(account::sign_up))
        // Type room
        .route("/public/type-room/list", get(type_room::list_type_room))
        // Amenity
        .route("/public/amenity/list", get(amenity::list_amenity))
        .with_state(db)
}
