use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::model::{database::Amenity, error::AppError, response::GeneralResponse};

pub async fn list_amenity(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("amenity")
        .select("*")
        .order("created_at.desc.nullsfirst")
        .execute()
        .await?;
    let query_result: Vec<Amenity> = query.json().await?;
    GeneralResponse::ok_with_result(query_result)
}
