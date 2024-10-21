use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::service::{account, reservation, room};

pub fn general_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/user/profile", get(account::get_profile))
        .route("/user/profile", post(account::update_profile))
        .route("/user/change-password", post(account::change_password))
        .route("/user/available-room", get(room::list_available_room))
        .route(
            "/user/reservation/add",
            post(reservation::general::add_reservation),
        )
        .with_state(db)
}
