use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::{GeneralStatus, User, UserGender, UserPosition},
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
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn update_user(
    State(db): State<Arc<Postgrest>>,
    Path(user_id): Path<String>,
    Json(mut input): Json<UpdateUser>,
) -> Result<GeneralResponse, AppError> {
    input.updated_at = Some(Utc::now());

    let update_user = serde_json::to_string(&input)?;
    let query = db
        .from("users")
        .eq("id", user_id)
        .update(update_user)
        .single()
        .execute()
        .await?;

    if !query.status().is_success() {
        let message = query.text().await?;
        return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message));
    }

    let mut user: User = query.json().await?;
    user.password = None;
    GeneralResponse::ok_with_result(user)
}
