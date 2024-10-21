use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::database::{GeneralStatus, ReservationStatus, TypeRoom};

pub mod admin;
pub mod customer;
pub mod general;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReservationOutput {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub room_id: Option<u64>,
    pub checkin_at: Option<DateTime<Utc>>,
    pub checkout_at: Option<DateTime<Utc>>,
    pub adult_number: Option<u32>,
    pub kid_number: Option<u32>,
    pub status: Option<ReservationStatus>,
    pub total_price: Option<u64>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub room: Option<RoomOutput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomOutput {
    pub id: Option<u64>,
    pub type_room_id: Option<u64>,
    pub room_number: Option<String>,
    pub floor: Option<u32>,
    pub status: Option<GeneralStatus>,
    pub updated_at: Option<DateTime<Utc>>,
    pub type_room: Option<TypeRoom>,
}
