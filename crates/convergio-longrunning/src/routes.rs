//! HTTP routes for long-running — executions, heartbeat, checkpoints, progress.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use convergio_db::pool::ConnPool;
use serde::Deserialize;
use serde_json::json;

/// Build all long-running routes.
pub fn longrunning_routes(pool: ConnPool) -> Router {
    Router::new()
        .route("/api/longrunning/heartbeat", post(register_heartbeat))
        .route("/api/longrunning/heartbeat/beat", post(beat))
        .route("/api/longrunning/heartbeat/stale", get(find_stale))
        .route(
            "/api/longrunning/checkpoint/:id",
            get(load_checkpoint).post(save_checkpoint),
        )
        .route(
            "/api/longrunning/checkpoint/:id/clear",
            delete(clear_checkpoint),
        )
        .route("/api/longrunning/progress/:id", get(load_progress))
        .route("/api/longrunning/delegation/:id", get(delegation_tree))
        .route(
            "/api/longrunning/delegation/:id/children",
            get(list_children),
        )
        .route("/api/longrunning/budget/:id", get(budget_status))
        .with_state(pool)
}

#[derive(Deserialize)]
struct RegisterReq {
    execution_id: String,
    interval_secs: u64,
}

async fn register_heartbeat(
    State(pool): State<ConnPool>,
    Json(r): Json<RegisterReq>,
) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    crate::heartbeat::register(&conn, &r.execution_id, r.interval_secs).map_err(map_err)?;
    ok(json!({"registered": r.execution_id}))
}

#[derive(Deserialize)]
struct BeatReq {
    execution_id: String,
}

async fn beat(State(pool): State<ConnPool>, Json(r): Json<BeatReq>) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    crate::heartbeat::beat(&conn, &r.execution_id).map_err(map_err)?;
    ok(json!({"ok": true}))
}

async fn find_stale(State(pool): State<ConnPool>) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    let stale = crate::heartbeat::find_stale(&conn).map_err(map_err)?;
    let entries: Vec<serde_json::Value> = stale
        .iter()
        .map(|(id, interval, age)| json!({"id": id, "interval": interval, "age_secs": age}))
        .collect();
    ok(json!(entries))
}

async fn save_checkpoint(
    State(pool): State<ConnPool>,
    Path(id): Path<String>,
    Json(state): Json<serde_json::Value>,
) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    crate::checkpoint::save(&conn, &id, &state).map_err(map_err)?;
    ok(json!({"saved": id}))
}

async fn load_checkpoint(
    State(pool): State<ConnPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    let cp = crate::checkpoint::load(&conn, &id).map_err(map_err)?;
    ok(json!({"checkpoint": cp}))
}

async fn clear_checkpoint(
    State(pool): State<ConnPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    let cleared = crate::checkpoint::clear(&conn, &id).map_err(map_err)?;
    ok(json!({"cleared": cleared}))
}

async fn load_progress(State(pool): State<ConnPool>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    let snap = crate::progress::load(&conn, &id).map_err(map_err)?;
    ok(json!(snap))
}

async fn delegation_tree(
    State(pool): State<ConnPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    let tree = crate::delegation::build_tree(&conn, &id).map_err(map_err)?;
    ok(json!(tree))
}

async fn list_children(State(pool): State<ConnPool>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    let children = crate::delegation::list_children(&conn, &id).map_err(map_err)?;
    ok(json!(children))
}

async fn budget_status(State(pool): State<ConnPool>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = pool.get().map_err(|e| map_err(e.into()))?;
    let (spent, limit) = crate::budget::status(&conn, &id).map_err(map_err)?;
    ok(json!({"spent_usd": spent, "limit_usd": limit}))
}

fn map_err(e: crate::types::LongRunError) -> (StatusCode, String) {
    match &e {
        crate::types::LongRunError::NotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
        crate::types::LongRunError::InvalidInput(_) => (StatusCode::BAD_REQUEST, e.to_string()),
        crate::types::LongRunError::BudgetExceeded { .. } => {
            (StatusCode::PAYMENT_REQUIRED, e.to_string())
        }
        // Do not leak internal details for DB/pool/json errors
        _ => {
            tracing::error!("longrunning route error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            )
        }
    }
}

fn ok(v: serde_json::Value) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Ok(Json(v))
}
