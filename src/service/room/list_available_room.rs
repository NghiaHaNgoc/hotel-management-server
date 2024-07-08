use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{database::Room, error::AppError, response::GeneralResponse};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct TimeRangeInput {
    time_from: DateTime<Utc>,
    time_to: DateTime<Utc>,
}

pub async fn list_available_room(
    State(db): State<Arc<Postgrest>>,
    Query(time_range): Query<TimeRangeInput>,
) -> Result<GeneralResponse, AppError> {
    let query_params_json = serde_json::to_string(&time_range)?;
    let query = db
        .rpc("available_room", query_params_json)
        .execute()
        .await?;
    let result: Vec<Room> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
