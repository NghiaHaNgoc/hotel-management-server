use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::model::{
    database::Amenity,
    error::AppError,
    response::GeneralResponse,
};

const QUERY_FIELD: [&str; 4] = ["id", "name", "type", "status"];

pub async fn list_amenity(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query_field = QUERY_FIELD.join(", ");
    let query = db.from("amenity").select(query_field).execute().await?;
    let query_result: Vec<Amenity> = query.json().await?;
    GeneralResponse::ok_with_result(query_result)
}
