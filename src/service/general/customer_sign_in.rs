use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::model::{
    database::User, error::AppError, response::GeneralResponse, token::create_token,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginData {
    email: String,
    password: String,
}

const QUERY_FIELD: [&str; 8] = [
    "id",
    "firstname",
    "surname",
    "email",
    "password",
    "position",
    "status",
    "link_avatar",
];

pub async fn customer_sign_in(
    State(db): State<Arc<Postgrest>>,
    Json(login_data): Json<LoginData>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("users")
        .select(QUERY_FIELD.join(", "))
        .eq("email", login_data.email)
        .eq("position", "4")
        .execute()
        .await?;
    let result_query: Vec<User> = query.json().await?;

    if result_query.len() == 1 {
        let user = result_query.get(0).unwrap();
        if let Some(status) = user.status {
            if status == 0 {
                let message = "This account is inactivated!".to_string();
                return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
            }
        } else {
            let message = "This account's status is not set!".to_string();
            return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message));
        }
        let result_verify = bcrypt::verify(login_data.password, user.password.as_ref().unwrap())?;

        if result_verify {
            let token = create_token(user)?;
            let result = json!({
                "firstname": user.firstname,
                "surname": user.surname,
                "position": user.position,
                "link_avatar": user.link_avatar,
                "token": token
            });
            GeneralResponse::ok_with_result(result)
        } else {
            let message = "Wrong password!".to_string();
            GeneralResponse::new_general(StatusCode::UNAUTHORIZED, Some(message))
        }
    } else {
        let message = "Email does not exist!".to_string();
        GeneralResponse::new_general(StatusCode::UNAUTHORIZED, Some(message))
    }
}
