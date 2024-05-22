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
    pub id_card: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub birth_day: Option<String>,
    pub gender: Option<UserGender>,
    pub position: Option<UserPosition>,
    pub salary: Option<f64>,
    pub link_avatar: Option<String>,
    pub password: Option<String>,
    pub status: Option<UserStatus>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UserPosition {
    Admin = 1,
    Housekeeper = 2,
    Receptionist = 3,
    Customer = 4
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UserGender {
    Male = 1,
    Female = 2
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UserStatus {
    Active = 1,
    Inactive= 0
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum PaymentStatus {
    Prepay = 1,
    Postpay = 2,
    Debit = 3
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ReservationStatus {
    Open = 1,
    Inprogress = 2,
    End = 3,
    Cancel = 4
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ServiceType {
    Food = 1,
    Drink = 2,
    Spa = 3
}
