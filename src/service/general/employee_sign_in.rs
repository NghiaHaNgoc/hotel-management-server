use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
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
    pub position: Option<i32>,
    pub salary: Option<f64>,
    pub link_avatar: Option<String>,
    pub password: Option<String>,
}

pub async fn employee_sign_in(
    State(db): State<Arc<Postgrest>>,
    Json(login_data): Json<LoginData>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("users")
        .select("firstname, surname,email,  password, position, status, link_avatar")
        .eq("email", login_data.email)
        .or("position.eq.1,or(position.eq.2, position.eq.3)")
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
            let token = create_token(user);
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
