use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::{GeneralStatus, UserGender, UserPosition},
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub birth_day: Option<String>,
    pub gender: Option<UserGender>,
    pub position: Option<UserPosition>,
    pub salary: Option<f64>,
    pub status: Option<GeneralStatus>,
}

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
    pub status: Option<GeneralStatus>,
}

pub async fn update_user(
    State(db): State<Arc<Postgrest>>,
    Path(user_id): Path<String>,
    Json(req_update_user): Json<UpdateUser>,
) -> Result<GeneralResponse, AppError> {
    let update_user = serde_json::to_string(&req_update_user)?;
    let query = db
        .from("users")
        .eq("id", user_id)
        .update(update_user)
        .execute()
        .await?;

    if !query.status().is_success() {
        let message = query.text().await?;
        return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message));
    }

    let result_query: Vec<ResponseUser> = query.json().await?;
    if result_query.len() == 1 {
        GeneralResponse::ok_with_result(result_query[0].clone())
    } else {
        GeneralResponse::new_general(StatusCode::NOT_MODIFIED, None)
    }
}
