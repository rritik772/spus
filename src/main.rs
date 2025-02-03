mod db;
mod schema;
mod utils;
mod service;

use std::collections::HashMap;
use axum::extract::{Path, Query, Request, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{routing::get, Json, Router};
use nanoid::nanoid;
use serde_json::json;
use serde_json::Value;
use tracing_subscriber::layer::SubscriberExt;

use utils::response::{generate_failure_response, generate_success_response};

const ROUTE_FN_TYPE: &str = "route";

#[tracing::instrument(
    name = "root",
    fields(request_id = %nanoid!(), fn_type = %ROUTE_FN_TYPE)
)]
async fn root() -> &'static str {
    "Hello, World!"
}

#[tracing::instrument(
    name = "health-check",
    fields(
        request_id = %nanoid!(),
        fn_type = %ROUTE_FN_TYPE
    )
)]
async fn health_check() -> Json<Value> {
    Json(json!({ "status": "Ok" }))
}

#[tracing::instrument(
    name = "db-health",
    fields(
        request_id = %nanoid!(),
        fn_type = %ROUTE_FN_TYPE
    ),
    skip(app_state)
)]
async fn db_health(
    State(app_state): State<utils::config::AppState>,
    req: Request
) -> (StatusCode, Json<Value>) {
    match db::get_connection(&app_state) {
        Some(_) => generate_success_response(
            json!({ "status": "Ok" }), 
            req.uri().to_string(), 
            None, None
        ),
        None => generate_failure_response(
            json!({"status": "failed"}), 
            req.uri().to_string(), 
            Some(StatusCode::NOT_FOUND), 
            None, None
        )
    }
}

#[tracing::instrument(
    name = "short-url",
    skip(app_state)
    fields(
        request_id = %nanoid!(),
        fn_type = %ROUTE_FN_TYPE
    )
)]
async fn short_url(
    State(app_state): State<utils::config::AppState>,
    Query(param): Query<HashMap<String, String>>,
    req: Request
) -> (StatusCode, Json<Value>) {
    let Some(url) = param.get("url") else {
        tracing::error!("Url not provided in the param.");
        return generate_failure_response(json!({ "msg": "url param not provided." }), req.uri().to_string(), Some(StatusCode::NOT_ACCEPTABLE), None, Some(true)) 
    };

    let Ok(_) = url::Url::parse(url) else {
        tracing::error!("Url {} does not look like url.", url);
        return generate_failure_response(
            json!({ "msg": format!("'{}' does not look like url.", url) }), 
            req.uri().to_string(), Some(StatusCode::NOT_ACCEPTABLE), None, Some(true)
        ) 
    };

    let Some(mut pool) = db::get_connection(&app_state) else {
        return generate_failure_response(json!({}), req.uri().to_string(), None, None, None);
    };

    service::short_url::short_url(&mut pool, url.into())
        .map_or_else(
            || generate_failure_response(json!({}), req.uri().to_string(), None, None, None),
            |resp| generate_success_response(json!({"data": resp}), req.uri().to_string(), None, None)
        )
}

#[tracing::instrument(
    name = "long-url",
    skip(app_state)
    fields(
        request_id = %nanoid!(),
        fn_type = %ROUTE_FN_TYPE
    )
)]
async fn long_url(
    State(app_state): State<utils::config::AppState>,
    Path(url): Path<String>
) ->  Redirect {
    let Some(mut pool) = db::get_connection(&app_state) else {
        return Redirect::permanent("/");
    };

    service::long_url::long_url(&mut pool, url.into())
        .map_or_else(
            || Redirect::permanent("/"),
            |resp| Redirect::permanent(&resp.original_url)
        )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_log::LogTracer::init().expect("Failed to set logger.");

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or(tracing_subscriber::EnvFilter::new("info"));
    let formatting_layer =
        tracing_bunyan_formatter::BunyanFormattingLayer::new("SPUS".into(), std::io::stdout);

    let subscriber = tracing_subscriber::registry::Registry::default()
        .with(env_filter)
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber.");

    utils::common_utils::check_envs()?;

    let app_state = utils::config::AppState::new().map_err(|e| {
        tracing::error!("Cannot get app state. E {:?}", e);

        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot make Application State.",
        )
    })?;

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/db-health", get(db_health))
        .route("/short", get(short_url))
        .route("/{url}", get(long_url))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        std::env::var("SERVER_HOST").unwrap_or("127.0.0.1".into()),
        std::env::var("SERVER_PORT").unwrap_or("5000".into())
    ))
    .await
    .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
