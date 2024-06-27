use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use postgrest::Postgrest;

use crate::model::{
    database::{GeneralStatus, User, UserPosition},
    response::GeneralResponse,
    token::Claims,
};

pub async fn authenticated_layer(
    claims: Claims,
    State(db): State<Arc<Postgrest>>,
    req: Request,
    next: Next,
) -> Response {
    let query = match db
        .from("users")
        .select("id")
        .eq("email", claims.email)
        .eq("position", (claims.position as u8).to_string())
        .eq("status", (GeneralStatus::Active as u8).to_string())
        .execute()
        .await
    {
        Ok(result) => result,
        Err(err) => {
            let message = format!("Err: {}", err);
            return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
                .into_response();
        }
    };

    if !query.status().is_success() {
        return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None)
            .into_response();
    }

    let result_query: Vec<User> = match query.json().await {
        Ok(result) => result,
        Err(err) => {
            let message = format!("Err: {}", err);
            return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
                .into_response();
        }
    };

    if result_query.len() == 1 {
        next.run(req).await
    } else {
        GeneralResponse::new_general(StatusCode::UNAUTHORIZED, None).into_response()
    }
}

pub async fn admin_layer(claims: Claims, req: Request, next: Next) -> Response {
    if claims.position == UserPosition::Admin {
        next.run(req).await
    } else {
        GeneralResponse::new_general(StatusCode::UNAUTHORIZED, None).into_response()
    }
}

pub async fn customer_layer(claims: Claims, req: Request, next: Next) -> Response {
    if claims.position == UserPosition::Customer {
        next.run(req).await
    } else {
        GeneralResponse::new_general(StatusCode::UNAUTHORIZED, None).into_response()
    }
}
