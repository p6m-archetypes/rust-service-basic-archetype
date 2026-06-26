use anyhow::Result;
use clap::Parser;

mod cli;
mod otel;
mod settings;

use {{ prefix_name }}_{{ suffix_name }}_core::{{ PrefixName }}{{ SuffixName }}Core;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = cli::Cli::parse();
    let settings = settings::Settings::load(cli.config.as_deref())?;

    // Structured JSON logging + OpenTelemetry tracing.
    // LOGGING_STRUCTURED=true → JSON format (injected via PlatformApplication spec.config).
    // OTEL_EXPORTER_OTLP_ENDPOINT → enables OTLP export (fail-open: absent = no traces).
    let structured = std::env::var("LOGGING_STRUCTURED")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    otel::init_tracing(structured);

    match cli.command {
        Some(cli::Commands::Config { action }) => match action {
            cli::ConfigAction::Defaults => {
                println!("{}", toml::to_string_pretty(&settings::Settings::default())?);
            }
            cli::ConfigAction::Show => {
                println!("{}", toml::to_string_pretty(&settings)?);
            }
        },
        None => {
            tracing::info!("Starting {{ project-name }}...");

            let core = {{ PrefixName }}{{ SuffixName }}Core::builder()
                .with_settings(&settings.core)
                .build()
                .await?;

            let svc_router = core.router();
            let mgmt_router = {{ PrefixName }}{{ SuffixName }}Core::management_router();

            // Service server — domain traffic on service_port
            let svc_addr = format!("{}:{}", settings.server.host, settings.server.port);
            let svc_listener = tokio::net::TcpListener::bind(&svc_addr).await?;
            tracing::info!("REST service listening on {svc_addr}");

            // Management server — health probes and metrics on management_port
            let mgmt_addr = format!("{}:{}", settings.server.host, settings.server.management_port);
            let mgmt_listener = tokio::net::TcpListener::bind(&mgmt_addr).await?;
            tracing::info!("Management server listening on {mgmt_addr}");

            tokio::select! {
                result = axum::serve(svc_listener, svc_router).with_graceful_shutdown(shutdown_signal()) => {
                    result?;
                }
                result = axum::serve(mgmt_listener, mgmt_router) => {
                    result?;
                }
            }
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to listen for ctrl-c");
    tracing::info!("Shutting down...");
}
