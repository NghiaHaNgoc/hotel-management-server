use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{
    database::{AmenityTypeRoom, GeneralStatus, ViewDirectionTypeRoom},
    error::AppError,
    response::GeneralResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResTypeRoom {
    id: Option<u64>,
    title: Option<String>,
    view_direction: Option<ViewDirectionTypeRoom>,
    preferential_services: Option<String>,
    size: Option<u64>,
    adult_capacity: Option<u32>,
    kids_capacity: Option<u32>,
    base_price: Option<u64>,
    status: Option<GeneralStatus>,
    created_at: Option<String>,
    updated_at: Option<String>,
    #[serde(skip_serializing)]
    amenity_type_room: Option<Vec<AmenityTypeRoom>>,
    amenities: Option<Vec<u64>>,
    images: Option<Vec<ResTypeRoomImage>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResTypeRoomImage {
    pub id: Option<u64>,
    pub link: Option<String>,
}

const QUERY_FIELD: [&str; 13] = [
    "id",
    "title",
    "view_direction",
    "preferential_services",
    "size",
    "adult_capacity",
    "kids_capacity",
    "base_price",
    "status",
    "created_at",
    "updated_at",
    "amenity_type_room(amenity_id)",
    "images: type_room_images(id, link)",
];

pub async fn list_type_room(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query_field = QUERY_FIELD.join(", ");
    let query = db.from("type_room").select(query_field).execute().await?;
    let mut res_type_room: Vec<ResTypeRoom> = query.json().await?;

    // Extract amenities
    for type_room in res_type_room.iter_mut() {
        if let Some(ref amenity_type_room) = type_room.amenity_type_room {
            type_room.amenities = Some(
                amenity_type_room
                    .into_iter()
                    .filter_map(|amenity| amenity.amenity_id)
                    .collect(),
            );
        }
    }

    GeneralResponse::ok_with_result(res_type_room)
}
