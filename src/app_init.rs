use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

use opentelemetry::KeyValue;
use opentelemetry_sdk::{
    runtime,
    trace::{BatchConfig, RandomIdGenerator, Sampler, Tracer},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use tracing_opentelemetry::OpenTelemetryLayer;

pub fn logging(level: Level, is_json: bool) {
    let subscriber = tracing_subscriber::fmt().with_max_level(level);
    if is_json {
        subscriber.json().init()
    } else {
        subscriber.init()
    };
}

pub fn tracing(level: Level) {
    let subscriber = registry()
        .with(
            OpenTelemetryLayer::new(init_tracer())
                .with_error_records_to_exceptions(true)
                .with_filter(LevelFilter::from_level(level)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_filter(LevelFilter::from_level(level)),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Could not setup tracing/logging");

    tracing::info!("Tracing resource {:?}", resource());
}

fn resource() -> Resource {
    Resource::new(vec![
        KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
        KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
    ])
}

// Construct Tracer for OpenTelemetryLayer
fn init_tracer() -> Tracer {
    use opentelemetry_otlp::TonicExporterBuilder;
    let exporter = TonicExporterBuilder::default();
    //let otlp_exporter = opentelemetry_otlp::new_exporter().http();
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                // Customize sampling strategy
                .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                    1.0,
                ))))
                // If export trace to AWS X-Ray, you can use XrayIdGenerator
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource()),
        )
        .with_batch_config(BatchConfig::default())
        .with_exporter(exporter)
        .install_batch(runtime::Tokio)
        .unwrap()
}
