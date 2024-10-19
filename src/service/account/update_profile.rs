use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::{User, UserGender},
    error::AppError,
    imgbb::ImgbbUploader,
    response::GeneralResponse,
    token::Claims,
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
    pub link_avatar: Option<String>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn update_profile(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
    Json(mut input): Json<UpdateUser>,
) -> Result<GeneralResponse, AppError> {
    if let Some(img_base64) = input.link_avatar {
        input.link_avatar = if img_base64.trim().is_empty() {
            None
        } else {
            let imgbb_res = ImgbbUploader::new(img_base64).upload().await?;
            imgbb_res.data.url
        };
    }

    input.updated_at = Some(Utc::now());

    let update_user = serde_json::to_string(&input)?;
    let query = db
        .from("users")
        .eq("email", claim.email)
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
