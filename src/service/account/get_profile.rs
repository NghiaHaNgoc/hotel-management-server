use crate::model::{
    database::{UserGender, UserPosition},
    error::AppError,
    response::GeneralResponse,
    token::Claims,
};
use axum::{extract::State, http::StatusCode};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub birth_day: Option<String>,
    pub gender: Option<UserGender>,
    pub position: Option<UserPosition>,
    pub salary: Option<f64>,
    pub link_avatar: Option<String>,
}

const QUERY_FIELD: [&str; 13] = [
    "firstname",
    "surname",
    "city",
    "district",
    "ward",
    "address",
    "phone",
    "email",
    "birth_day",
    "gender",
    "position",
    "salary",
    "link_avatar",
];
pub async fn get_profile(
    State(db): State<Arc<Postgrest>>,
    claim: Claims,
) -> Result<GeneralResponse, AppError> {
    let query_field = QUERY_FIELD.join(", ");
    let query = db
        .from("users")
        .select(query_field)
        .eq("email", claim.email)
        .execute()
        .await?;
    let result_query: Vec<ResponseUser> = query.json().await?;
    if result_query.len() == 1 {
        GeneralResponse::ok_with_result(result_query[0].clone())
    } else {
        GeneralResponse::new_general(StatusCode::NOT_FOUND, None)
    }
}
