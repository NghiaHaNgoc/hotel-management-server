use std::{env, time::{self, Duration, SystemTime}};

use super::{database::User, error::AppError, response::GeneralResponse};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub email: String,
    pub position: i32,
    pub exp: u128,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = GeneralResponse;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        // GeneralResponse::new_general(status_code, message)
        let TypedHeader(Authorization(bearer)) =
            match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
                Ok(header) => header,
                Err(err) => {
                    let status_code = axum::http::StatusCode::UNAUTHORIZED;
                    let message = err.to_string();
                    let res = GeneralResponse::new_general(status_code, Some(message)).unwrap();
                    return Err(res);
                }
            };
        let secret_key = env::var("JWT_KEY").expect("JWT_KEY must be set!");

        // Decode the user data
        let token_data = match decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(err) => {
                let status_code = axum::http::StatusCode::BAD_REQUEST;
                let message = err.to_string();
                let res = GeneralResponse::new_general(status_code, Some(message)).unwrap();
                return Err(res);
            }
        };

        Ok(token_data.claims)
    }
}

const HOUR_TO_SECOND: u64 = 60 * 60;

pub fn create_token(user: &User) -> String {
    let email = match user.email.as_ref() {
        Some(email) => email.clone(),
        None => String::new()
    };
    let position = user.position.unwrap_or_default();
    let now = SystemTime::now();
    let exp_after = Duration::from_secs(HOUR_TO_SECOND * 12);
    let exp = (now + exp_after)
        .duration_since(time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let jwt_key = env::var("JWT_KEY").expect("JWT_KEY must be set!");
    let claims = Claims {
        email,
        position,
        exp,
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_key.as_bytes()),
    )
    .unwrap();
    token
}
