pub struct TracingGuard {
    #[cfg(feature = "otel")]
    otel_provider: Option<opentelemetry_sdk::trace::TracerProvider>,
    #[cfg(feature = "otel")]
    _tokio_rt: tokio::runtime::Runtime,
}

impl Drop for TracingGuard {
    fn drop(&mut self) {
        #[cfg(feature = "otel")]
        if let Some(provider) = self.otel_provider.take() {
            use opentelemetry::trace::TracerProvider as _;
            // Run shutdown inside block_on so the tokio runtime is active while
            // the batch processor's final flush and HTTP export complete.
            self._tokio_rt.block_on(async move {
                provider.shutdown().ok();
            });
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
    let (registry, otel_provider, tokio_rt) = {
        use opentelemetry_otlp::WithExportConfig;

        let tokio_rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("failed to build tokio runtime for OTel export");

        let _enter = tokio_rt.enter();

        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_endpoint("http://localhost:4318/v1/traces")
            .build()
            .expect("failed to build OTLP span exporter");

        let resource = opentelemetry_sdk::Resource::new([opentelemetry::KeyValue::new(
            "service.name",
            "open-gl-rs",
        )]);

        let batch_config = opentelemetry_sdk::trace::BatchConfigBuilder::default()
            .with_scheduled_delay(std::time::Duration::from_millis(500))
            .with_max_queue_size(8192)
            .build();

        let batch_processor = opentelemetry_sdk::trace::BatchSpanProcessor::builder(
            exporter,
            opentelemetry_sdk::runtime::Tokio,
        )
        .with_batch_config(batch_config)
        .build();

        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_resource(resource)
            .with_span_processor(batch_processor)
            .build();

        let tracer = opentelemetry::trace::TracerProvider::tracer(&provider, "new-open-gl-rs");
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        (registry.with(otel_layer), provider, tokio_rt)
    };

    registry.init();

    TracingGuard {
        #[cfg(feature = "otel")]
        otel_provider: Some(otel_provider),
        #[cfg(feature = "otel")]
        _tokio_rt: tokio_rt,
    }
}
