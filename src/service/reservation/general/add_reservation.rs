use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    model::{
        database::{Room, UserPosition},
        error::AppError,
        response::GeneralResponse,
        token::Claims,
    },
    service::reservation::ReservationOutput,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddReservationInput {
    user_id: Option<u64>,
    #[serde(skip_deserializing)]
    room_id: Option<u64>,
    adult_number: u32,
    kid_number: u32,
    checkin_at: DateTime<Utc>,
    checkout_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    type_room_id: u64,
    total_price: Option<u64>,
}

pub async fn add_reservation(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
    Json(mut added_reservation): Json<AddReservationInput>,
) -> Result<GeneralResponse, AppError> {
    let rooms = available_room(&db, &added_reservation).await?;
    let room;
    if let Some(r) = rooms.first() {
        room = r;
    } else {
        let message = "No available room!".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }
    added_reservation.room_id = room.id;
    if claim.position == UserPosition::Customer || added_reservation.user_id.is_none() {
        added_reservation.user_id = Some(claim.id);
    }
    let reservation_json = serde_json::to_string(&added_reservation)?;
    let query = db
        .from("reservations")
        .insert(reservation_json)
        .select("*, room(*, type_room(*))")
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let data: ReservationOutput = query.json().await?;
        GeneralResponse::ok_with_result(data)
    } else {
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None)
    }
}

async fn available_room(
    db: &Arc<Postgrest>,
    added_reservation: &AddReservationInput,
) -> Result<Vec<Room>, AppError> {
    let query_params_json = json!({
        "time_from": added_reservation.checkin_at,
        "time_to": added_reservation.checkout_at
    })
    .to_string();

    let query = db
        .rpc("available_room", query_params_json)
        .select("*, type_room!inner(*)")
        .eq("type_room_id", added_reservation.type_room_id.to_string());
    //.eq(
    //    "type_room.adult_capacity",
    //    added_reservation.adult_number.to_string(),
    //)
    //.eq(
    //    "type_room.kid_number",
    //    added_reservation.kid_number.to_string(),
    //);

    let query = query.execute().await?;
    let result: Vec<Room> = query.json().await?;
    Ok(result)
}
