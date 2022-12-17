use axum::http::StatusCode;
use axum::{routing::get, Extension, Router};
use prometheus::{Registry, TextEncoder};
use std::net::SocketAddr;

const METRICS_ROUTE: &str = "/metrics";

pub fn start_prometheus_server(addr: SocketAddr) -> RegistryService {
    let registry = Registry::new();

    let registry_service = RegistryService::new(registry);

    let app = Router::new()
        .route(METRICS_ROUTE, get(metrics))
        .layer(Extension(registry_service.clone()));

    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    registry_service
}

async fn metrics(Extension(registry_service): Extension<RegistryService>) -> (StatusCode, String) {
    let metrics_families = registry_service.gather_all();
    match TextEncoder.encode_to_string(&metrics_families) {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unable to encode metrics: {error}"),
        ),
    }
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[derive(Clone)]
pub struct RegistryService {
    registry: Registry,
}

impl RegistryService {
    pub fn new(registry: Registry) -> Self {
        RegistryService { registry }
    }

    pub fn gather_all(&self) -> Vec<prometheus::proto::MetricFamily> {
        self.registry.gather()
    }
}
