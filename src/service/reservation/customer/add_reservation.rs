use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    model::{
        database::{Reservation, UserPosition},
        database_error::SupabaseError,
        error::AppError,
        response::GeneralResponse,
        token::Claims,
    },
    utils::{is_ovelaped_date_range, valid_time_from_and_to},
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddReservationInput {
    #[serde(skip_deserializing)]
    user_id: Option<u64>,
    room_id: u64,
    checkin_at: String,
    checkout_at: String,
}

pub async fn add_reservation(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
    Json(mut added_reservation): Json<AddReservationInput>,
) -> Result<GeneralResponse, AppError> {
    // let validate_user_query = db
    //     .from("users")
    //     .select("position")
    //     .eq("id", claim.position)
    //     .single()
    //     .execute()
    //     .await?;
    // if validate_user_query.status().is_success() {
    //     let user: User = validate_user_query.json().await?;
    //     if let Some(position) = user.position {
    //         if position != UserPosition::Customer {
    //             let message = "User position is not customer.".to_string();
    //             return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    //         }
    //     } else {
    //         let message = "User position is not found.".to_string();
    //         return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message));
    //     }
    // } else {
    //     let message = "User is not found.".to_string();
    //     return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    // }

    added_reservation.user_id = Some(claim.id);

    if claim.position != UserPosition::Customer {
        let message = "User position is not customer.".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    let validate_time_range_query = db
        .from("reservations")
        .select("checkin_at, checkout_at")
        .eq("room_id", added_reservation.room_id.to_string())
        .execute()
        .await?;
    let reserved_result: Vec<Reservation> = validate_time_range_query.json().await?;

    if reserved_result.is_empty() {
        if !valid_time_from_and_to(
            &added_reservation.checkin_at,
            &added_reservation.checkout_at,
        )? {
            let message = "Time from is equal or greater than time to.".to_string();
            return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
        }
    }

    for reservated in reserved_result {
        if let Some(checkin_at_reserved) = reservated.checkin_at {
            if let Some(checkout_at_reserved) = reservated.checkout_at {
                if is_ovelaped_date_range(
                    (
                        &added_reservation.checkin_at,
                        &added_reservation.checkout_at,
                    ),
                    (&checkin_at_reserved, &checkout_at_reserved),
                )? {
                    let message =
                        "This reservation for this room is overlaped with other reservations."
                            .to_string();
                    return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
                }
            }
        }
    }

    let added_reservation_json = serde_json::to_string(&added_reservation)?;
    let query = db
        .from("reservations")
        .insert(added_reservation_json)
        .single()
        .execute()
        .await?;

    let query_status = query.status();
    if query_status.is_success() {
        GeneralResponse::new_general(StatusCode::OK, None)
    } else if query_status == reqwest::StatusCode::CONFLICT {
        let message = "Room is not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    } else {
        let db_error: SupabaseError = query.json().await?;
        GeneralResponse::new(StatusCode::INTERNAL_SERVER_ERROR, db_error)
    }
}
