use axum::{Json, extract::State, http::{header, StatusCode}, response::IntoResponse};
use serde::Serialize;
use crate::AppState;

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

/// Prometheus metrics endpoint.
/// Returns metrics collected by the `metrics` crate via `metrics-exporter-prometheus`.
pub async fn metrics() -> impl IntoResponse {
    // TODO: wire up metrics-exporter-prometheus handle and return rendered text.
    // For now returns an empty valid Prometheus response so Kubernetes scraping succeeds.
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        "# Prometheus metrics\n",
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
