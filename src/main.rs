mod utils;

use axum::{routing::get, Json, Router};
use nanoid::nanoid;
use serde_json::json;
use serde_json::Value;
use tracing_subscriber::layer::SubscriberExt;

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

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check));

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
