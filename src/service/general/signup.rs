use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{database::User, error::AppError, response::GeneralResponse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupUser {
    pub firstname: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub gender: String,
}

pub async fn signup(
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

    // Verify gender
    if !signup_user.gender.eq("male") && !signup_user.gender.eq("female") {
        let message = "Invalid gender!".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    // Hash password
    signup_user.password = bcrypt::hash(signup_user.password, bcrypt::DEFAULT_COST)?;

    let signup_user_str = serde_json::to_string(&signup_user)?;
    let query = db.from("users").insert(signup_user_str).execute().await?;

    if query.status().is_success() {
        GeneralResponse::new_general(StatusCode::OK, None)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
    }
}
