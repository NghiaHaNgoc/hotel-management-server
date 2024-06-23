use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::model::{database::Room, error::AppError, response::GeneralResponse};

pub async fn list_room(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("room")
        .select("*")
        .order("id.asc.nullsfirst")
        .execute()
        .await?;
    let result: Vec<Room> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
