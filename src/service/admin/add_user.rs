use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
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
    pub position: UserPosition,
    pub salary: Option<f64>,
    pub password: String,
}

pub async fn add_user(
    State(db): State<Arc<Postgrest>>,
    Json(mut added_employee): Json<AddEmployeeRequest>,
) -> Result<GeneralResponse, AppError> {
    // Validate salary
    if let Some(salary) = added_employee.salary {
        if salary < 0.0 {
            let message = "Invalid salary!".to_string();
            return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
        }
    }

    // Validate salary for customer
    if added_employee.position == UserPosition::Customer && added_employee.salary.is_some() {
        let message = "No salary for customer!".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }
    // Verify existed email
    let query_verify = db
        .from("users")
        .select("id")
        .eq("email", added_employee.email.as_str())
        .execute()
        .await?;
    let result_verify: Vec<User> = query_verify.json().await?;
    if result_verify.len() != 0 {
        let message = "Email has been used!".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    // Hash password
    added_employee.password = bcrypt::hash(added_employee.password, bcrypt::DEFAULT_COST)?;

    // Insert to db
    let added_employee_str = serde_json::to_string(&added_employee)?;
    let query = db
        .from("users")
        .insert(added_employee_str)
        .execute()
        .await?;

    if query.status().is_success() {
        GeneralResponse::new_general(StatusCode::OK, None)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
    }
}
