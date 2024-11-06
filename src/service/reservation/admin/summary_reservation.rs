use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::{
    model::{error::AppError, response::GeneralResponse},
    service::reservation::ReservationOutput,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryReservation {
    total_end_reservation: Option<u64>,
    total_customer: Option<u64>,
    total_revenue: Option<u64>,
    most_booked_room_type: Option<String>
}

pub async fn summary_reservation(
    State(db): State<Arc<Postgrest>>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .rpc("get_reservation_summary", r#"{}"#)
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let result: SummaryReservation = query.json().await?;
        GeneralResponse::ok_with_result(result)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
