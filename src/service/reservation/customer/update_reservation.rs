use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    model::{
        database::{ReservationStatus, Room},
        error::AppError,
        response::GeneralResponse,
        token::Claims,
    },
    service::reservation::ReservationOutput,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateReservationInput {
    adult_number: Option<u32>,
    kid_number: Option<u32>,
    checkin_at: Option<DateTime<Utc>>,
    checkout_at: Option<DateTime<Utc>>,
    total_price: Option<u64>,
    #[serde(skip_deserializing)]
    room_id: Option<u64>,
    #[serde(skip)]
    type_room_id: Option<u64>,
    #[serde(skip_deserializing)]
    updated_at: Option<DateTime<Utc>>,
}

pub async fn update_reservation(
    State(db): State<Arc<Postgrest>>,
    Path(reservation_id): Path<u64>,
    claims: Claims,
    Json(mut input): Json<UpdateReservationInput>,
) -> Result<GeneralResponse, AppError> {
    let mut time_changed = false;
    // Check if change time
    if input.checkin_at.is_some() || input.checkout_at.is_some() {
        let check_query = db
            .from("reservations")
            .select("*, room(*)")
            .eq("id", reservation_id.to_string())
            .eq("user_id", claims.id.to_string())
            .eq("status", (ReservationStatus::Waiting as u8).to_string())
            .single()
            .execute()
            .await?;
        if !check_query.status().is_success() {
            let message = "Id not found or not in valid status!".to_string();
            return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
        }
        let reservation_before: ReservationOutput = check_query.json().await?;
        if input.checkin_at != reservation_before.checkin_at
            || input.checkout_at != reservation_before.checkout_at
        {
            time_changed = true;
            if let Some(room) = reservation_before.room {
                input.type_room_id = room.type_room_id;
            }
        }
    }

    // Asign new room if time changed
    if time_changed {
        let rooms = available_room(&db, &input).await?;
        if let Some(room) = rooms.get(0) {
            input.room_id = room.id;
        } else {
            let message = "Out of rooms at this time please select other time!".to_string();
            return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
        }
    }

    input.updated_at = Some(Utc::now());

    let input_json = serde_json::to_string(&input)?;
    let query = db
        .from("reservations")
        .eq("id", reservation_id.to_string())
        .update(input_json)
        .select("*, room(*, type_room(*))")
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let result: ReservationOutput = query.json().await?;
        GeneralResponse::ok_with_result(result)
    } else {
        let message = "Id not found or not in valid status!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

async fn available_room(
    db: &Arc<Postgrest>,
    added_reservation: &UpdateReservationInput,
) -> Result<Vec<Room>, AppError> {
    let query_params_json = json!({
        "time_from": added_reservation.checkin_at,
        "time_to": added_reservation.checkout_at,
        "updated_reservation_id": added_reservation.type_room_id
    })
    .to_string();

    let query = db
        .rpc("available_room_in_update_reservation", query_params_json)
        .select("*")
        .eq(
            "type_room_id",
            added_reservation
                .type_room_id
                .map(|id| id.to_string())
                .unwrap_or_default(),
        )
        .execute()
        .await?;

    let result: Vec<Room> = query.json().await?;
    Ok(result)
}
