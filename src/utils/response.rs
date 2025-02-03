use axum::{http::StatusCode, Json};
use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
pub struct ApplicationResponse {
    pub url: String,
    pub time: NaiveDateTime,
    pub is_error: bool,
    pub data: Value,
    pub status_code: u16
}

impl ApplicationResponse {
    #[tracing::instrument(name = "Application Response", fields(fn_type = "default"))]
    pub fn default() -> Value {
        serde_json::to_value(&Self {
            url: "/".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            is_error: true,
            data: json!("{}"),
            time: Utc::now().naive_utc(),
        })
        .expect("Failed to convert to json value.")
    }
}

#[tracing::instrument(name = "generate_success_response")]
pub fn generate_success_response(
    data: Value, 
    url: String,
    status: Option<StatusCode>,
    time: Option<NaiveDateTime>
) -> (StatusCode, Json<Value>) {
    let status_code = status.unwrap_or(StatusCode::OK);
    let time = time.unwrap_or(Utc::now().naive_utc());

    tracing::debug!(
        "msg: {:?}, url: {:?}, status: {:?}, time: {:?}",
        data, url, status, time
    );

    (
        status_code,
        Json(serde_json::to_value(ApplicationResponse {
            url,
            time,
            data,
            status_code: status_code.as_u16(),
            is_error: false
        }).unwrap_or_else(|e|  {
            tracing::error!("Error while create response: ${:?}", e);
            return ApplicationResponse::default()
        }))
    )
}

#[tracing::instrument(name = "generate_failure_response")]
pub fn generate_failure_response(
    data: Value, 
    url: String,
    status: Option<StatusCode>,
    time: Option<NaiveDateTime>,
    is_error: Option<bool>
) -> (StatusCode, Json<Value>) {
    let status_code = status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let time = time.unwrap_or(Utc::now().naive_utc());
    let is_error = is_error.unwrap_or(true);

    tracing::debug!(
        "msg: {:?}, url: {:?}, status: {:?}, time: {:?}",
        data, url, status, time
    );

    (
        status_code,
        Json(serde_json::to_value(ApplicationResponse {
            url,
            time,
            data,
            status_code: status_code.as_u16(),
            is_error
        }).unwrap_or_else(|e| {
            tracing::error!("Error while create response: ${:?}", e);
            return ApplicationResponse::default()
        }))
    )
}