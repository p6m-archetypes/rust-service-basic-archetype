use crate::AppState;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn readiness() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn liveness() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// Prometheus metrics endpoint: renders everything the installed recorder has collected.
pub async fn metrics(handle: metrics_exporter_prometheus::PrometheusHandle) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        handle.render(),
    )
}

#[derive(Serialize)]
pub struct ServiceInfo {
    service: &'static str,
    status: &'static str,
}

/// Root endpoint — identifies the service. Replace with real application routes.
pub async fn root(State(_state): State<AppState>) -> Json<ServiceInfo> {
    Json(ServiceInfo {
        service: "{{ project-name }}",
        status: "ok",
    })
}
