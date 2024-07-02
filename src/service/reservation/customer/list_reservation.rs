use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::model::{
    database::Reservation, error::AppError, response::GeneralResponse, token::Claims,
};

pub async fn list_reservation(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("reservations")
        .select("*")
        .eq("user_id", claim.id.to_string())
        .order("checkin_at.asc.nullsfirst")
        .execute()
        .await?;
    let result: Vec<Reservation> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
