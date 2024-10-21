use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;

use crate::model::{error::AppError, response::GeneralResponse};

use super::ResTypeRoom;

pub async fn list_type_room(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("type_room")
        .select("*, amenity_type_room(*), images: type_room_images(*)")
        .order("updated_at.desc.nullsfirst")
        .execute()
        .await?;
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
