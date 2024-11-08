use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    model::{error::AppError, response::GeneralResponse},
    service::reservation::ReservationOutput,
};

pub async fn summary_reservation(
    State(db): State<Arc<Postgrest>>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .rpc("get_reservation_summary", r#"{}"#)
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let result: Value = query.json().await?;
        GeneralResponse::ok_with_result(result)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
