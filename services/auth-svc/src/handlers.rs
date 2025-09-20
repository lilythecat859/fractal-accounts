use axum::{extract::State, Json};
use serde_json::json;
use sqlx::PgPool;

pub async fn register(
    State(_pool): State<PgPool>,
) -> Json<serde_json::Value> {
    Json(json!({ "msg": "use gRPC endpoint" }))
}

pub async fn login(
    State(_pool): State<PgPool>,
) -> Json<serde_json::Value> {
    Json(json!({ "msg": "use gRPC endpoint" }))
}
