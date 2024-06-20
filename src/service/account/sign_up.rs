use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::model::{
    database::{User, UserGender},
    error::AppError,
    response::GeneralResponse,
    token,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupUser {
    pub firstname: String,
    pub surname: String,
    pub city: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: String,
    pub birth_day: Option<String>,
    pub gender: Option<UserGender>,
    pub password: String,
}
pub async fn sign_up(
    State(db): State<Arc<Postgrest>>,
    Json(mut signup_user): Json<SignupUser>,
) -> Result<GeneralResponse, AppError> {
    // Verify existed email
    let query_verify = db
        .from("users")
        .select("id")
        .eq("email", signup_user.email.as_str())
        .execute()
        .await?;
    let result_verify: Vec<User> = query_verify.json().await?;
    if result_verify.len() != 0 {
        let message = "Email has been used!".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    // Hash password
    signup_user.password = bcrypt::hash(signup_user.password, bcrypt::DEFAULT_COST)?;

    // Insert to db
    let signup_user_str = serde_json::to_string(&signup_user)?;
    let query = db.from("users").insert(signup_user_str).execute().await?;

    if query.status().is_success() {
        let result_query: Vec<User> = query.json().await?;
        let user = result_query.get(0).unwrap();
        let token = token::create_token(user)?;

        let result = json!({
            "firstname": user.firstname,
            "surname": user.surname,
            "position": user.position,
            "link_avatar": user.link_avatar,
            "token": token
        });
        GeneralResponse::ok_with_result(result)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
    }
}
