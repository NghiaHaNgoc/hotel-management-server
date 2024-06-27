use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{
    database::{GeneralStatus, TypeRoom},
    error::AppError,
    response::GeneralResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomDetail {
    pub id: Option<u64>,
    pub type_room_id: Option<u64>,
    pub room_number: Option<String>,
    pub floor: Option<u32>,
    pub status: Option<GeneralStatus>,
    pub updated_at: Option<String>,
    pub reservations: Option<Vec<Reserved>>,
    pub type_room: Option<TypeRoom>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reserved {
    checkin_at: Option<String>,
    checkout_at: Option<String>,
}

const QUERY_FIELD: [&str; 7] = [
    "id",
    "type_room_id",
    "room_number",
    "status",
    "updated_at",
    "reservations(checkin_at, checkout_at)",
    "type_room(*)",
];

pub async fn room_detail_of_customer(
    State(db): State<Arc<Postgrest>>,
    Path(room_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query_field = QUERY_FIELD.join(", ");
    let query = db
        .from("room")
        .select(query_field)
        .eq("id", room_id.to_string())
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let result: RoomDetail = query.json().await?;
        GeneralResponse::ok_with_result(result)
    } else {
        let message = "room_id is not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
