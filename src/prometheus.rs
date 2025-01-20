use std::collections::HashMap;

use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

pub fn prometheus_middleware() -> PrometheusMetrics {
    let mut labels = HashMap::new();
    labels.insert("app_name".to_string(), "xythrion-api".to_string());

    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap()
}
