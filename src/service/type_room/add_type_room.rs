use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
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
pub struct ReqAddTypeRoom {
    title: Option<String>,
    view_direction: Option<ViewDirectionTypeRoom>,
    preferential_services: Option<String>,
    size: Option<u64>,
    adult_capacity: Option<u32>,
    kids_capacity: Option<u32>,
    base_price: Option<u64>,
    #[serde(skip_serializing)]
    amenities: Option<Vec<u64>>,
    #[serde(skip_serializing)]
    images: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqAddTypeRoomImage {
    pub type_room_id: Option<u64>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqAddTypeRoomAmenity {
    type_room_id: Option<u64>,
    amenity_id: Option<u64>,
}

pub async fn add_type_room(
    State(db): State<Arc<Postgrest>>,
    Json(mut added_type_room): Json<ReqAddTypeRoom>,
) -> Result<GeneralResponse, AppError> {
    let mut type_room_images: Vec<ReqAddTypeRoomImage> = Vec::new();
    let mut type_room_amenities: Vec<ReqAddTypeRoomAmenity> = Vec::new();

    // Handle extract and validate amenity
    if let Some(ref amenities) = added_type_room.amenities {
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
                .map(|amenity| ReqAddTypeRoomAmenity {
                    type_room_id: None,
                    amenity_id: Some(*amenity),
                })
                .collect();
        }
    }

    // Handle upload images
    if let Some(arr_img_base64) = added_type_room.images {
        for img_base64 in arr_img_base64 {
            let imgbb_res = ImgbbUploader::new(img_base64).upload().await?;
            let type_room_image = ReqAddTypeRoomImage {
                type_room_id: None,
                link: imgbb_res.data.url,
            };
            type_room_images.push(type_room_image);
        }
        added_type_room.images = None;
    }

    // Handle add type room
    let json_added_type_room = serde_json::to_string(&added_type_room)?;
    let query = db
        .from("type_room")
        .insert(json_added_type_room)
        .execute()
        .await?;
    let result_type_rooms: Vec<TypeRoom> = query.json().await?;
    if result_type_rooms.len() != 1 {
        return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None);
    }

    let result_type_room = result_type_rooms[0].clone();

    // Handle type room id for images
    for type_room_image in type_room_images.iter_mut() {
        type_room_image.type_room_id = result_type_room.id;
    }

    // Handle type room id for amenities
    for type_room_amenity in type_room_amenities.iter_mut() {
        type_room_amenity.type_room_id = result_type_room.id;
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
    GeneralResponse::new_general(StatusCode::OK, None)
}
