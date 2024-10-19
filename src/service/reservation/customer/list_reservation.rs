use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::{model::{
    error::AppError, response::GeneralResponse, token::Claims,
}, service::reservation::ReservationOutput};

pub async fn list_reservation(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("reservations")
        .select("*, room(*, type_room(*))")
        .eq("user_id", claim.id.to_string())
        .order("updated_at.desc.nullsfirst")
        .execute()
        .await?;
    let result: Vec<ReservationOutput> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
