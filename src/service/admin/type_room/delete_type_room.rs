use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{database::TypeRoom, error::AppError, response::GeneralResponse};

pub async fn delete_type_room(
    State(db): State<Arc<Postgrest>>,
    Path(type_room_id): Path<String>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("type_room")
        .eq("id", type_room_id)
        .delete()
        .execute()
        .await?;
    let deleted_type_room: Vec<TypeRoom> = query.json().await?;
    if deleted_type_room.is_empty() {
        let message = "Id type room not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    } else {
        GeneralResponse::new_general(StatusCode::OK, None)
    }
}
