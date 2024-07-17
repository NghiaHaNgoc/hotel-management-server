
use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::{model::{
    error::AppError, response::GeneralResponse,
}, service::reservation::ReservationOutput};

pub async fn list_reservation(
    State(db): State<Arc<Postgrest>>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("reservations")
        .select("*, room(*, type_room(*))")
        .order("checkin_at.asc.nullsfirst")
        .execute()
        .await?;
    let result: Vec<ReservationOutput> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
