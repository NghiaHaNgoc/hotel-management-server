use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::{GeneralStatus, Room}, database_error::SupabaseError, error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRoomReq {
    pub type_room_id: Option<u64>,
    pub room_number: Option<String>,
    pub floor: Option<u32>,
    pub status: Option<GeneralStatus>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn update_room(
    State(db): State<Arc<Postgrest>>,
    Path(room_id): Path<u64>,
    Json(mut input): Json<UpdateRoomReq>,
) -> Result<GeneralResponse, AppError> {
    input.updated_at = Some(Utc::now());
    let updated_room_json = serde_json::to_string(&input)?;
    let query = db
        .from("room")
        .eq("id", room_id.to_string())
        .single()
        .update(updated_room_json)
        .execute()
        .await?;

    let query_status = query.status();
    if query.status().is_success() {
        let room: Room = query.json().await?;
        GeneralResponse::ok_with_result(room)
    } else {
        match query_status {
            reqwest::StatusCode::CONFLICT => {
                let message = "type_room_id not found!".to_string();
                GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
            }
            reqwest::StatusCode::NOT_ACCEPTABLE => {
                let message = "room_id not found!".to_string();
                GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
            }
            _ => {
                let db_error: SupabaseError = query.json().await?;
                GeneralResponse::new(StatusCode::INTERNAL_SERVER_ERROR, db_error)
            }
        }
    }
}
