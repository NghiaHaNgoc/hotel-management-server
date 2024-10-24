use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use postgrest::Postgrest;

use crate::model::{
    database::{Reservation, ReservationStatus},
    error::AppError,
    response::GeneralResponse,
    token::Claims,
};

pub async fn cancel_reservation(
    State(db): State<Arc<Postgrest>>,
    Path(reservation_id): Path<u64>,
    claim: Claims,
) -> Result<GeneralResponse, AppError> {
    let now = Utc::now().to_rfc3339();
    let query = db
        .from("reservations")
        .eq("id", reservation_id.to_string())
        .eq("user_id", claim.id.to_string())
        .in_("status", [
            (ReservationStatus::Waiting as u8).to_string(),
            (ReservationStatus::Open as u8).to_string(),
            (ReservationStatus::Inprogress as u8).to_string()
        ])
        .update(format!("{{ \"status\": 5, \"updated_at\": \"{}\" }}", now))
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let data: Reservation = query.json().await?;

        GeneralResponse::ok_with_result(data)
    } else {
        let message = "Your reservation not found or not in valid status!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
