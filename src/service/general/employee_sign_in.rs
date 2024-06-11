use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::model::{
    database::{GeneralStatus, User},
    error::AppError,
    response::GeneralResponse,
    token::create_token,
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

pub async fn employee_sign_in(
    State(db): State<Arc<Postgrest>>,
    Json(login_data): Json<LoginData>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("users")
        .select(QUERY_FIELD.join(", "))
        .eq("email", login_data.email)
        .or("position.eq.1,or(position.eq.2, position.eq.3)")
        .execute()
        .await?;
    let result_query: Vec<User> = query.json().await?;

    if result_query.len() == 1 {
        let user = result_query[0].clone();
        match user.status {
            Some(status) if status == GeneralStatus::Inactive => {
                let message = "This account is inactivated!".to_string();
                return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
            }
            _ => (),
        }

        let result_verify = bcrypt::verify(login_data.password, user.password.as_ref().unwrap())?;

        if result_verify {
            let token = create_token(&user)?;
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
