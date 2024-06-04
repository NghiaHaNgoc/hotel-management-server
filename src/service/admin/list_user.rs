use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{
    database::{UserGender, UserPosition, UserStatus},
    error::AppError,
    response::GeneralResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
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
    pub status: Option<UserStatus>,
}

pub async fn list_user(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query = db.from("users").select("*").execute().await?;
    let query_result: Vec<ResponseUser> = query.json().await?;
    GeneralResponse::ok_with_result(query_result)
}
