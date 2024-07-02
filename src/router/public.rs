use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::service::{account, room};

pub fn public_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/test", get(room::list_available_room))
        .route("/employee/sign-in", post(account::employee_sign_in))
        .route("/customer/sign-in", post(account::customer_sign_in))
        .route("/customer/sign-up", post(account::sign_up))
        .with_state(db)
}
