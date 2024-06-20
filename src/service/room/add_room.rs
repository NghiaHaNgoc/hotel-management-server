use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::GeneralStatus, database_error::SupabaseError, error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddRoomReq {
    pub type_room_id: Option<u64>,
    pub room_number: Option<String>,
    pub floor: Option<u32>,
    pub status: Option<GeneralStatus>,
}

pub async fn add_room(
    State(db): State<Arc<Postgrest>>,
    Json(added_room): Json<AddRoomReq>,
) -> Result<GeneralResponse, AppError> {
    let added_room_json = serde_json::to_string(&added_room)?;
    let query = db
        .from("room")
        .insert(added_room_json)
        .single()
        .execute()
        .await?;

    let query_status = query.status();
    if query_status.is_success() {
        GeneralResponse::new_general(StatusCode::OK, None)
    } else if query_status == reqwest::StatusCode::CONFLICT {
        let message = "type_room_id not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    } else {
        let db_error: SupabaseError = query.json().await?;
        GeneralResponse::new(StatusCode::INTERNAL_SERVER_ERROR, db_error)
    }
}
