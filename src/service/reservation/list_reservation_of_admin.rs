use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::model::{database::Reservation, error::AppError, response::GeneralResponse};

pub async fn list_reservation_of_admin(
    State(db): State<Arc<Postgrest>>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("reservations")
        .select("*")
        .order("checkin_at.asc.nullsfirst")
        .execute()
        .await?;
    let result: Vec<Reservation> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
