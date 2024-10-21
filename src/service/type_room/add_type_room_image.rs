use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::TypeRoomImage, error::AppError, imgbb::ImgbbUploader, response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqAddTypeRoomImage {
    link: String,
    type_room_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqAddTypeRoomAmenity {
    type_room_id: Option<u64>,
    amenity_id: Option<u64>,
}

pub async fn add_type_room_image(
    State(db): State<Arc<Postgrest>>,
    Path(type_room_id): Path<u64>,
    Json(mut added_type_room_image): Json<ReqAddTypeRoomImage>,
) -> Result<GeneralResponse, AppError> {
    let imgbb_res = ImgbbUploader::new(added_type_room_image.link)
        .upload()
        .await?;
    added_type_room_image.link = imgbb_res.data.url.unwrap_or_default();
    added_type_room_image.type_room_id = Some(type_room_id);
    let input_json = serde_json::to_string(&added_type_room_image)?;

    let query = db
        .from("type_room_images")
        .insert(input_json)
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let result: TypeRoomImage = query.json().await?;
        GeneralResponse::ok_with_result(result)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
