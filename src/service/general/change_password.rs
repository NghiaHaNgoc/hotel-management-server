use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::DEFAULT_COST;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::model::{database::User, error::AppError, response::GeneralResponse, token::Claims};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangePasswordData {
    old_password: String,
    new_password: String,
}

pub async fn change_password(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
    Json(change_password_data): Json<ChangePasswordData>,
) -> Result<GeneralResponse, AppError> {
    // Verify old password
    let verify_query = db
        .from("users")
        .select("id, password")
        .eq("id", claims.id.to_string())
        .execute()
        .await?;
    let verify_query_result: Vec<User> = verify_query.json().await?;
    if verify_query_result.len() != 1 {
        let message = "user not found!".to_string();
        let err = AppError::new(message);
        return Err(err);
    };
    let user = &verify_query_result[0];
    let result_verify = bcrypt::verify(
        change_password_data.old_password,
        user.password.as_ref().unwrap(),
    )?;

    // Update new password
    if result_verify {
        let password_hash = bcrypt::hash(change_password_data.new_password, DEFAULT_COST)?;
        let update_password_query = json!({
            "password": password_hash
        });
        let query = db
            .from("users")
            .eq("id", claims.id.to_string())
            .update(update_password_query.to_string())
            .execute()
            .await?;
        if query.status().is_success() {
            GeneralResponse::new_general(StatusCode::OK, None)
        } else {
            let message = query.text().await?;
            let err = AppError::new(message);
            Err(err)
        }
    } else {
        let message = "Wrong old password!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}
