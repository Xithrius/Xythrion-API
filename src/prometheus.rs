use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

pub fn prometheus_middleware() -> PrometheusMetrics {
    PrometheusMetricsBuilder::new("xythrion_api")
        .endpoint("/metrics")
        .build()
        .unwrap()
}
