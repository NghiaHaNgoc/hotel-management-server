use std::sync::Arc;

use axum::extract::State;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{
    database::{GeneralStatus, User, UserGender, UserPosition},
    error::AppError,
    response::GeneralResponse,
};

pub async fn list_user(State(db): State<Arc<Postgrest>>) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("users")
        .select("*")
        .order("updated_at.desc.nullsfirst")
        .execute()
        .await?;
    let mut users: Vec<User> = query.json().await?;
    for user in users.iter_mut() {
        user.password = None;
    }
    GeneralResponse::ok_with_result(users)
}
