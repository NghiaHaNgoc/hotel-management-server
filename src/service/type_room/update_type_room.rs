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

use crate::{
    model::{
        database::{Amenity, ViewDirectionTypeRoom},
        error::AppError,
        response::GeneralResponse,
    },
    utils::vector_difference,
};

use super::ResTypeRoom;

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
    #[serde(skip_serializing)]
    amenities: Option<Vec<u64>>,
    #[serde(skip_serializing)]
    images: Option<Vec<String>>,
    #[serde(skip_deserializing)]
    updated_at: Option<DateTime<Utc>>,
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
    Json(mut input): Json<ReqUpdateTypeRoom>,
) -> Result<GeneralResponse, AppError> {
    if let Some(ref amenities) = input.amenities {
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
        }
    }

    // Handle delete type room images
    if input.images.is_some() {
        db.from("type_room_images")
            .eq("type_room_id", type_room_id.to_string())
            .delete()
            .execute()
            .await?;
    }
    // Handle add type room images
    if let Some(ref images) = input.images {
        if !images.is_empty() {
            let images: Vec<ReqUpdateRoomImage> = images
                .into_iter()
                .map(|image| ReqUpdateRoomImage {
                    type_room_id: Some(type_room_id),
                    link: Some(image.to_string()),
                })
                .collect();

            let images_json = serde_json::to_string(&images)?;
            let query = db
                .from("type_room_images")
                .insert(images_json)
                .execute()
                .await?;

            if !query.status().is_success() {
                let message = query.text().await?;
                return GeneralResponse::new_general(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(message),
                );
            }
        }
    }

    // Handle delete type room amenities
    if input.amenities.is_some() {
        db.from("amenity_type_room")
            .eq("type_room_id", type_room_id.to_string())
            .delete()
            .execute()
            .await?;
    }

    // Handle add type room amenities
    if let Some(ref amenities) = input.amenities {
        if !amenities.is_empty() {
            let amenities: Vec<ReqUpdateTypeRoomAmenity> = amenities
                .into_iter()
                .map(|amenity| ReqUpdateTypeRoomAmenity {
                    type_room_id: Some(type_room_id),
                    amenity_id: Some(*amenity),
                })
                .collect();
            let amenities_json = serde_json::to_string(&amenities)?;

            let query = db
                .from("amenity_type_room")
                .insert(amenities_json)
                .execute()
                .await?;
            if !query.status().is_success() {
                let message = query.text().await?;
                return GeneralResponse::new_general(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(message),
                );
            }
        }
    }

    // Handle update type room
    input.updated_at = Some(Utc::now());
    let json_updated_type_room = serde_json::to_string(&input)?;
    let query = db
        .from("type_room")
        .eq("id", type_room_id.to_string())
        .update(json_updated_type_room)
        .select("*, amenity_type_room(*), type_room_images(*)")
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let mut type_room: ResTypeRoom = query.json().await?;
        if let Some(ref amenity_type_room) = type_room.amenity_type_room {
            type_room.amenities = Some(
                amenity_type_room
                    .into_iter()
                    .filter_map(|amenity| amenity.amenity_id)
                    .collect(),
            );
        }
        if let Some(ref type_room_images) = type_room.type_room_images {
            type_room.images = Some(
                type_room_images
                    .iter()
                    .filter_map(|image| image.link.as_ref().map(|link| link.to_owned()))
                    .collect(),
            );
        }

        GeneralResponse::ok_with_result(type_room)
    } else {
        GeneralResponse::new_general(
            StatusCode::NOT_FOUND,
            Some("Type room not found!".to_string()),
        )
    }
}
