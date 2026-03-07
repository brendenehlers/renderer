pub struct TracingGuard {
    #[cfg(feature = "otel")]
    otel_provider: opentelemetry_sdk::trace::TracerProvider,
}

impl Drop for TracingGuard {
    fn drop(&mut self) {
        #[cfg(feature = "otel")]
        {
            use opentelemetry::trace::TracerProvider as _;
            self.otel_provider.shutdown().ok();
        }
    }
}

pub fn init() -> TracingGuard {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false);

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    #[cfg(feature = "tracy")]
    let registry = registry.with(tracing_tracy::TracyLayer::default().with_filter(
        tracing_subscriber::filter::filter_fn(|meta| meta.target().starts_with("new_open_gl_rs")),
    ));

    #[cfg(feature = "otel")]
    let (registry, otel_provider) = {
        use opentelemetry_otlp::WithExportConfig;

        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_endpoint("http://localhost:4318/v1/traces")
            .build()
            .expect("failed to build OTLP span exporter");

        let resource = opentelemetry_sdk::Resource::new([opentelemetry::KeyValue::new(
            "service.name",
            "open-gl-rs",
        )]);

        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_resource(resource)
            .with_simple_exporter(exporter)
            .build();

        let tracer = opentelemetry::trace::TracerProvider::tracer(&provider, "new-open-gl-rs");
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        (registry.with(otel_layer), provider)
    };

    registry.init();

    TracingGuard {
        #[cfg(feature = "otel")]
        otel_provider,
    }
}
