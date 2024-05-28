use std::sync::Arc;

use axum::{routing::post, Router};
use postgrest::Postgrest;

use crate::service::general;

pub fn public_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/employee/sign-in", post(general::employee_sign_in))
        .route("/customer/sign-in", post(general::customer_sign_in))
        .route("/customer/sign-up", post(general::sign_up))
        .with_state(db)
}
