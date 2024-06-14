use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    model::{
        database::{Amenity, TypeRoom, ViewDirectionTypeRoom},
        error::AppError,
        imgbb::ImgbbUploader,
        response::GeneralResponse,
    },
    utils::vector_difference,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqUpdateTypeRoom {
    title: Option<String>,
    view_direction: Option<ViewDirectionTypeRoom>,
    preferential_services: Option<String>,
    size: Option<u64>,
    adult_capacity: Option<u32>,
    kids_capacity: Option<u32>,
    base_price: Option<u64>,
    #[serde(skip_deserializing)]
    updated_at: Option<String>,
    #[serde(skip_serializing)]
    amenities: Option<Vec<u64>>,
    #[serde(skip_serializing)]
    add_images: Option<Vec<String>>,
    #[serde(skip_serializing)]
    delete_images: Option<Vec<u64>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqUpdateRoomImage {
    pub type_room_id: Option<u64>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqUpdateTypeRoomAmenity {
    type_room_id: Option<u64>,
    amenity_id: Option<u64>,
}

pub async fn update_type_room(
    State(db): State<Arc<Postgrest>>,
    Path(type_room_id): Path<u64>,
    Json(mut updated_type_room): Json<ReqUpdateTypeRoom>,
) -> Result<GeneralResponse, AppError> {
    let mut type_room_images: Vec<ReqUpdateRoomImage> = Vec::new();
    let mut type_room_amenities: Vec<ReqUpdateTypeRoomAmenity> = Vec::new();

    if let Some(ref amenities) = updated_type_room.amenities {
        if !amenities.is_empty() {
            let amenities_str: Vec<String> = amenities
                .iter()
                .map(|amenity| amenity.to_string())
                .collect();
            let validate_amenity_query = db
                .from("amenity")
                .select("id")
                .in_("id", amenities_str)
                .execute()
                .await?;
            let available_amenities: Vec<Amenity> = validate_amenity_query.json().await?;
            let available_amenities: Vec<u64> = available_amenities
                .into_iter()
                .filter_map(|amenity| amenity.id)
                .collect();
            let validate_diff = vector_difference(&amenities, &available_amenities);
            if !validate_diff.is_empty() {
                let message = format!("Amenites ID {:?} is not found!", validate_diff);
                return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
            }
            type_room_amenities = amenities
                .into_iter()
                .map(|amenity| ReqUpdateTypeRoomAmenity {
                    type_room_id: Some(type_room_id),
                    amenity_id: Some(*amenity),
                })
                .collect();
        }
    }

    // Handle upload images
    if let Some(arr_img_base64) = updated_type_room.add_images {
        for img_base64 in arr_img_base64 {
            let imgbb_res = ImgbbUploader::new(img_base64).upload().await?;
            let type_room_image = ReqUpdateRoomImage {
                type_room_id: Some(type_room_id),
                link: imgbb_res.data.url,
            };
            type_room_images.push(type_room_image);
        }
        updated_type_room.add_images = None;
    }

    // Handle delete type room images
    if let Some(ref delete_images) = updated_type_room.delete_images {
        if !delete_images.is_empty() {
            let delete_images_str: Vec<String> =
                delete_images.iter().map(|id| id.to_string()).collect();
            db.from("type_room_images")
                .eq("type_room_id", type_room_id.to_string())
                .in_("id", delete_images_str)
                .delete()
                .execute()
                .await?;
        }
    }

    // Handle add type room images
    if !type_room_images.is_empty() {
        let json_type_room_images = serde_json::to_string(&type_room_images)?;
        let query = db
            .from("type_room_images")
            .insert(json_type_room_images)
            .execute()
            .await?;
        if !query.status().is_success() {
            let message = query.text().await?;
            return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message));
        }
    }

    // Handle delete type room amenities
    if updated_type_room.amenities.is_some() {
        db.from("amenity_type_room")
            .eq("type_room_id", type_room_id.to_string())
            .delete()
            .execute()
            .await?;
    }

    // Handle add type room amenities
    if !type_room_amenities.is_empty() {
        let json_type_room_amenities = serde_json::to_string(&type_room_amenities)?;
        let query = db
            .from("amenity_type_room")
            .insert(json_type_room_amenities)
            .execute()
            .await?;
        if !query.status().is_success() {
            let message = query.text().await?;
            return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message));
        }
    }

    // Handle update type room
    updated_type_room.updated_at = Some(Utc::now().to_rfc3339());
    let json_updated_type_room = serde_json::to_string(&updated_type_room)?;
    let query = db
        .from("type_room")
        .eq("id", type_room_id.to_string())
        .update(json_updated_type_room)
        .execute()
        .await?;
    let result_type_rooms: Vec<TypeRoom> = query.json().await?;
    if result_type_rooms.len() != 1 {
        return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None);
    }
    GeneralResponse::new_general(StatusCode::OK, None)
}
