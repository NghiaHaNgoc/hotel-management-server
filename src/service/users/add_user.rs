use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use rand::{distributions, Rng};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::{
    database::{User, UserGender, UserPosition},
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddEmployeeRequest {
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: String,
    pub birth_day: Option<String>,
    pub gender: Option<UserGender>,
    pub position: UserPosition,
    pub salary: Option<f64>,
    pub password: Option<String>,
}

pub async fn add_user(
    State(db): State<Arc<Postgrest>>,
    Json(mut input): Json<AddEmployeeRequest>,
) -> Result<GeneralResponse, AppError> {
    if input.position == UserPosition::Admin {
        let message = "Invalid position!".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    // Validate salary
    if let Some(salary) = input.salary {
        if salary < 0.0 {
            let message = "Invalid salary!".to_string();
            return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
        }
    }

    // Validate salary for customer
    if input.position == UserPosition::Customer && input.salary.is_some() {
        input.salary = None;
        //let message = "No salary for customer!".to_string();
        //return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }
    // Verify existed email
    let query_verify = db
        .from("users")
        .select("id")
        .eq("email", input.email.as_str())
        .execute()
        .await?;
    let result_verify: Vec<User> = query_verify.json().await?;
    if result_verify.len() != 0 {
        let message = "Email has been used!".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    // Generate password
    let password: String = rand::thread_rng()
        .sample_iter(distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    input.password = bcrypt::hash(password.as_str(), bcrypt::DEFAULT_COST).ok();

    // Insert to db
    let added_employee_str = serde_json::to_string(&input)?;
    let query = db
        .from("users")
        .insert(added_employee_str)
        .single()
        .execute()
        .await?;

    if query.status().is_success() {
        let mut user: User = query.json().await?;
        user.password = Some(password);
        GeneralResponse::ok_with_result(user)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
    }
}
