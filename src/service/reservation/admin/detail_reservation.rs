use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::{
    model::{error::AppError, response::GeneralResponse},
    service::reservation::ReservationOutput,
};

pub async fn detail_reservation(
    State(db): State<Arc<Postgrest>>,
    Path(reservation_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("reservations")
        .select("*, room(*, type_room(*))")
        .eq("id", reservation_id.to_string())
        .single()
        .execute()
        .await?;

    if query.status().is_success() {
        let reservation: ReservationOutput = query.json().await?;

        GeneralResponse::ok_with_result(reservation)
    } else {
        let message = "Reservation not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
