mod add_type_room;
mod add_type_room_image;
mod delete_type_room;
mod list_type_room;
mod update_type_room;

pub use add_type_room::add_type_room;
pub use add_type_room_image::add_type_room_image;
use chrono::{DateTime, Utc};
pub use delete_type_room::delete_type_room;
pub use list_type_room::list_type_room;
use serde::{Deserialize, Serialize};
pub use update_type_room::update_type_room;

use crate::model::database::{AmenityTypeRoom, GeneralStatus, TypeRoomImage, ViewDirectionTypeRoom};

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
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    amenity_type_room: Option<Vec<AmenityTypeRoom>>,
    amenities: Option<Vec<u64>>,
    #[serde(skip_serializing)]
    type_room_images: Option<Vec<TypeRoomImage>>,
    images: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResTypeRoomImage {
    pub id: Option<u64>,
    pub link: Option<String>,
}
