use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode};
use postgrest::Postgrest;

use crate::model::{database::Room, database_error::SupabaseError, error::AppError, response::GeneralResponse};

pub async fn delete_room(
    State(db): State<Arc<Postgrest>>,
    Path(room_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query = db.from("room").eq("id", room_id.to_string()).single().execute().await?;
    if query.status().is_success() {
        GeneralResponse::new_general(StatusCode::OK, None)
    } else {
        let db_error: SupabaseError = query.json().await?;
        GeneralResponse::new(StatusCode::BAD_REQUEST, db_error)
    }
}
