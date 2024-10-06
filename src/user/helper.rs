use axum::http::HeaderMap;
use axum::{Json};
use axum::http::StatusCode;
use crate::common::response::ErrorResponse;
use crate::common::types::Status;
use crate::config::CONFIG;

pub fn validate_security_key(headers: &HeaderMap) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let security_key = headers.get("X-Vronix-Security").and_then(|h| h.to_str().ok());
    if let Some(key) = security_key {
        if key != CONFIG.vronix_security_key {
            let error_response = ErrorResponse {
                status: Status::Failure,
                message: "유효하지 않은 보안 키입니다.".to_string(),
            };
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    } else {
        let error_response = ErrorResponse {
            status: Status::Failure,
            message: "보안 키가 제공되지 않았습니다.".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    Ok(())
}