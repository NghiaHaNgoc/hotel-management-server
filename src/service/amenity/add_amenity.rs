use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::{Amenity, AmenityType, GeneralStatus},
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddAmenity {
    name: String,
    #[serde(rename = "type")]
    amenity_type: AmenityType,
    status: Option<GeneralStatus>,
}
pub async fn add_amenity(
    State(db): State<Arc<Postgrest>>,
    Json(added_amenity): Json<AddAmenity>,
) -> Result<GeneralResponse, AppError> {
    let json_added_type_room = serde_json::to_string(&added_amenity)?;
    let query = db
        .from("amenity")
        .insert(json_added_type_room)
        .single()
        .execute()
        .await?;

    if query.status().is_success() {
        let amenity: Amenity = query.json().await?;
        GeneralResponse::ok_with_result(amenity)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
    }
}
