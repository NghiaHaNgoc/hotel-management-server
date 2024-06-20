use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{database::Amenity, error::AppError, response::GeneralResponse};

pub async fn delete_amenity(
    State(db): State<Arc<Postgrest>>,
    Path(amenity_id): Path<String>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("amenity")
        .eq("id", amenity_id)
        .delete()
        .execute()
        .await?;

    if query.status().is_success() {
        let deleted_amenity: Vec<Amenity> = query.json().await?;
        if deleted_amenity.is_empty() {
            let message = "id not found!".to_string();
            GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
        } else {
            GeneralResponse::new_general(StatusCode::OK, None)
        }
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
    }
}
