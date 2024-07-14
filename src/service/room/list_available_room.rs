use std::sync::Arc;

use axum::extract::{Query, State};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::model::{database::Room, error::AppError, response::GeneralResponse};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct QueryInput {
    time_from: Option<DateTime<Utc>>,
    time_to: Option<DateTime<Utc>>,
    type_room: Option<u64>,
    adult_capacity: Option<u32>,
    kids_capacity: Option<u32>,
}

pub async fn list_available_room(
    State(db): State<Arc<Postgrest>>,
    Query(query_input): Query<QueryInput>,
) -> Result<GeneralResponse, AppError> {
    let query_params_json = if query_input.time_from.is_some() || query_input.time_to.is_some() {
        json!({
            "time_from": query_input.time_from,
            "time_to": query_input.time_to
        })
        .to_string()
    } else {
        json!({

            "time_from": "",
            "time_to": ""
        })
        .to_string()
    };

    let mut query = db
        .rpc("available_room", query_params_json)
        .select("*, type_room!inner(*)");
    if let Some(type_room) = query_input.type_room {
        query = query.eq("type_room_id", type_room.to_string());
    }
    if let Some(adult_capacity) = query_input.adult_capacity {
        println!("{}", adult_capacity);
        query = query.gte("type_room.adult_capacity", adult_capacity.to_string());
    }
    if let Some(kid_capacity) = query_input.kids_capacity {
        query = query.gte("type_room.kids_capacity", kid_capacity.to_string());
    }

    let query = query.execute().await?;
    let result: Vec<Room> = query.json().await?;
    GeneralResponse::ok_with_result(result)
}
