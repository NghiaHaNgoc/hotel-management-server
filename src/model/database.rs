use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub birth_day: Option<String>,
    pub gender: Option<UserGender>,
    pub position: Option<UserPosition>,
    pub salary: Option<f64>,
    pub link_avatar: Option<String>,
    pub password: Option<String>,
    pub status: Option<GeneralStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeRoom {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub view_direction: Option<ViewDirectionTypeRoom>,
    pub preferential_services: Option<String>,
    pub size: Option<u64>,
    pub adult_capacity: Option<u32>,
    pub kids_capacity: Option<u32>,
    pub base_price: Option<u64>,
    pub status: Option<GeneralStatus>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeRoomImage {
    pub id: Option<u64>,
    pub type_room_id: Option<u64>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Amenity {
    pub id: Option<u64>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub amenity_type: Option<AmenityType>,
    pub status: Option<GeneralStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmenityTypeRoom {
    pub id: Option<u64>,
    pub type_room_id: Option<u64>,
    pub amenity_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: Option<u64>,
    pub type_room_id: Option<u64>,
    pub room_number: Option<String>,
    pub floor: Option<u32>,
    pub status: Option<GeneralStatus>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reservation {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub room_id: Option<u64>,
    pub checkin_at: Option<String>,
    pub checkout_at: Option<String>,
    pub status: Option<ReservationStatus>,
    pub updated_at: Option<String>,
}
/// ----------------------------------------------------------------------------
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum GeneralStatus {
    Active = 1,
    Inactive = 0,
}

/// NOTE: User
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum UserPosition {
    Admin = 1,
    Housekeeper = 2,
    Receptionist = 3,
    Customer = 4,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum UserGender {
    Male = 1,
    Female = 2,
}

/// NOTE: Payment
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PaymentStatus {
    Prepay = 1,
    Postpay = 2,
    Debit = 3,
}

/// NOTE: Reservation
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ReservationStatus {
    Open = 1,
    Inprogress = 2,
    End = 3,
    Cancel = 4,
}

/// NOTE: Type Room
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ViewDirectionTypeRoom {
    River = 1,
    City = 2,
}

/// NOTE: Amenity
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum AmenityType {
    General = 1,
    Bathroom = 2,
    Other = 3,
}
