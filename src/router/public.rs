use std::sync::Arc;

use axum::{routing::post, Router};
use postgrest::Postgrest;

use crate::service::account;

pub fn public_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/employee/sign-in", post(account::employee_sign_in))
        .route("/customer/sign-in", post(account::customer_sign_in))
        .route("/customer/sign-up", post(account::sign_up))
        .with_state(db)
}
