use crate::{handlers, AppState};
use axum::{routing::get, Router};
use metrics_exporter_prometheus::PrometheusBuilder;

/// Main service router — application routes.
pub fn router(state: AppState) -> Router {
    Router::new().route("/", get(handlers::root)).with_state(state)
}

/// Management router — health and metrics endpoints on management_port.
/// Kept separate from the service router so Kubernetes network policy can
/// restrict probe traffic independently from service traffic.
/// Installs the process-global Prometheus recorder (call once, at startup).
pub fn management_router() -> Router {
    let metrics_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus metrics recorder");

    // Seed a build-info family so /metrics is meaningful from the first scrape.
    metrics::gauge!(
        "{{ project_name }}_build_info",
        "version" => env!("CARGO_PKG_VERSION")
    )
    .set(1.0);

    Router::new()
        .route("/health/readiness", get(handlers::readiness))
        .route("/health/liveness", get(handlers::liveness))
        .route(
            "/metrics",
            get(move || {
                let handle = metrics_handle.clone();
                async move { handlers::metrics(handle).await }
            }),
        )
}
