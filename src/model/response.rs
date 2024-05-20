use super::error::AppError;
use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

mod response_message;

#[derive(Debug, Clone)]
pub struct GeneralResponse {
    pub status_code: StatusCode,
    pub body: String,
}

// NOTE: General response for all layer and handler
impl GeneralResponse {
    pub fn new<T: Serialize>(
        status_code: StatusCode,
        result: T,
    ) -> Result<GeneralResponse, AppError> {
        let message = response_message::OK.to_string();
        let body_obj = GeneralBody::new(status_code, message, Some(result));
        let body = serde_json::to_string(&body_obj)?;

        let res = GeneralResponse { status_code, body };
        Ok(res)
    }

    pub fn new_general(
        status_code: StatusCode,
        message: Option<String>,
    ) -> Result<GeneralResponse, AppError> {
        let message = if let Some(msg) = message {
            msg
        } else {
            get_general_message(&status_code)
        };

        // let status_message = StatusMessage::new(status_code, message);
        let general_body = GeneralBody::<bool>::new(status_code, message, None);
        let body = serde_json::to_string(&general_body)?;

        let res = GeneralResponse { status_code, body };
        Ok(res)
    }

    pub fn ok_with_result<T: Serialize>(result: T) -> Result<GeneralResponse, AppError> {
        let status_code = StatusCode::OK;
        let general_body =
            GeneralBody::new(status_code, get_general_message(&status_code), Some(result));
        let body = serde_json::to_string(&general_body)?;

        let res = GeneralResponse { status_code, body };
        Ok(res)
    }
}

impl IntoResponse for GeneralResponse {
    fn into_response(self) -> axum::response::Response {
        let mut header = HeaderMap::new();
        header.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        (self.status_code, header, self.body).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralBody<T> {
    result: Option<T>,
    status_code: u16,
    message: String,
}

impl<T: Serialize> GeneralBody<T> {
    pub fn new(status_code: StatusCode, message: String, result: Option<T>) -> GeneralBody<T> {
        let status_code = status_code.as_u16();
        GeneralBody {
            result,
            status_code,
            message,
        }
    }
}

fn get_general_message(status_code: &StatusCode) -> String {
    match status_code {
        &StatusCode::OK => response_message::OK,
        &StatusCode::UNAUTHORIZED => response_message::UNAUTHORIZED,
        &StatusCode::INTERNAL_SERVER_ERROR => response_message::INTERNAL_SERVER_ERROR,
        &StatusCode::NOT_FOUND => response_message::NOT_FOUND,
        &StatusCode::BAD_REQUEST => response_message::BAD_REQUEST,
        &StatusCode::NOT_MODIFIED => response_message::NOT_MODIFIED,
        _ => response_message::UNDEFINED,
    }
    .to_string()
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct StatusMessage {
//     status_code: u16,
//     message: String,
// }
//
// impl StatusMessage {
//     pub fn new(status_code: StatusCode, message: String) -> StatusMessage {
//         let code_status = status_code.as_u16();
//         StatusMessage {
//             status_code: code_status,
//             message,
//         }
//     }
// }
