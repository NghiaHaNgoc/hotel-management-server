use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{error::AppError, response::GeneralResponse};

pub async fn delete_room(
    State(db): State<Arc<Postgrest>>,
    Path(room_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("room")
        .eq("id", room_id.to_string())
        .single()
        .delete()
        .execute()
        .await?;

    let query_status = query.status();
    if query_status.is_success() {
        GeneralResponse::new_general(StatusCode::OK, None)
    } else {
        let message = "room_id not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
