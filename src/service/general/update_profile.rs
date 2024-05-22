use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::{UserGender, UserPosition},
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
    pub id_card: Option<String>,
    pub phone: Option<String>,
    pub birth_day: Option<String>,
    pub gender: Option<UserGender>,
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
    pub gender: Option<UserGender>,
    pub position: Option<UserPosition>,
    pub salary: Option<f64>,
    pub link_avatar: Option<String>,
}

pub async fn update_profile(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
    Json(mut update_user): Json<UpdateUser>,
) -> Result<GeneralResponse, AppError> {
    if let Some(img_base64) = update_user.link_avatar {
        let imgbb_res = ImgbbUploader::new(img_base64).upload().await?;
        update_user.link_avatar = imgbb_res.data.url;
    }

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
