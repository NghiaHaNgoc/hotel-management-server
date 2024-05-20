use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::DEFAULT_COST;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    error::AppError, imgbb::ImgbbUploader, response::GeneralResponse, token::Claims,
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
    pub id_card: Option<String>,
    pub phone: Option<String>,
    pub birth_day: Option<String>,
    pub gender: Option<String>,
    pub link_avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
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
    pub gender: Option<String>,
    pub position: Option<u32>,
    pub salary: Option<f64>,
    pub link_avatar: Option<String>,
}

const RESPONSE_USER_FIELD: [&str; 7] = [
    "firstname",
    "surname",
    "city",
    "district",
    "ward",
    "address",
    "id_card",
];

pub async fn update_profile(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
    Json(mut update_user): Json<UpdateUser>,
) -> Result<GeneralResponse, AppError> {
    // Validate gender
    if let Some(gender) = update_user.gender {
        if !gender.eq("male") && !gender.eq("female") {
            let err_message = "Invalid gender!".to_string();
            let err = AppError::new(err_message);
            return Err(err);
        }
        update_user.gender = Some(gender);
    }

    update_user = upload_avatar(update_user).await?;

    let update_user = serde_json::to_string(&update_user)?;
    let query = db
        .from("users")
        .eq("email", claim.email)
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

async fn upload_avatar(mut update_user: UpdateUser) -> Result<UpdateUser, AppError> {
    if let Some(data) = update_user.link_avatar {
        let imgbb_res = ImgbbUploader::new(data).upload().await?;
        update_user.link_avatar = imgbb_res.data.url;
        Ok(update_user)
    } else {
        Ok(update_user)
    }
}
