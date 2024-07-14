use serde::{Deserialize, Serialize};

use crate::model::database::{ReservationStatus, Room};

pub mod customer;
pub mod general;
pub mod admin;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReservationOutput {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub room_id: Option<u64>,
    pub checkin_at: Option<String>,
    pub checkout_at: Option<String>,
    pub status: Option<ReservationStatus>,
    pub total_price: Option<u64>,
    pub updated_at: Option<String>,
    pub room: Option<Room>
}
