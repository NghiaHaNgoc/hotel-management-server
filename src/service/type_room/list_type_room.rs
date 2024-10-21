use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::model::{error::AppError, response::GeneralResponse};

use super::ResTypeRoom;

pub async fn list_type_room(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("type_room")
        .select("*, amenity_type_room(*), type_room_images(*)")
        .order("updated_at.desc.nullsfirst")
        .execute()
        .await?;
    let mut type_rooms: Vec<ResTypeRoom> = query.json().await?;

    // Extract amenities and images
    for type_room in type_rooms.iter_mut() {
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
    }

    GeneralResponse::ok_with_result(type_rooms)
}
