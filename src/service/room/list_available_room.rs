use std::sync::Arc;

use axum::extract::{Query, State};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::model::{database::Room, error::AppError, response::GeneralResponse};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct TimeRangeInput {
    time_from: DateTime<Utc>,
    time_to: DateTime<Utc>,
    type_room: Option<u64>,
}

pub async fn list_available_room(
    State(db): State<Arc<Postgrest>>,
    Query(time_range): Query<TimeRangeInput>,
) -> Result<GeneralResponse, AppError> {
    let query_params_json = json!({
        "time_from": time_range.time_from,
        "time_to": time_range.time_to
    })
    .to_string();

    let mut query = db.rpc("available_room", query_params_json);
    if let Some(type_room) = time_range.type_room {
        query = query.eq("type_room", type_room.to_string());
    }

    let query = query.execute().await?;
    let result: Vec<Room> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
