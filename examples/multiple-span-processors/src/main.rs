use opentelemetry_api::global::{self, shutdown_tracer_provider};
use opentelemetry_api::trace::{mark_span_as_active, TraceError, Tracer};
use opentelemetry_api::KeyValue;
use opentelemetry_sdk::trace::{BatchSpanProcessor, Config, TracerProvider};
use opentelemetry_sdk::Resource;
use opentelemetry_stdout::SpanExporter as StdoutExporter;
use std::time::Duration;

fn init_tracer() -> Result<(), TraceError> {
    // build a jaeger batch span processor
    let jaeger_processor = BatchSpanProcessor::builder(
        opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("trace-demo")
            .with_trace_config(
                Config::default()
                    .with_resource(Resource::new(vec![KeyValue::new("exporter", "jaeger")])),
            )
            .build_async_agent_exporter(opentelemetry_sdk::runtime::Tokio)?,
        opentelemetry_sdk::runtime::Tokio,
    )
    .build();

    // build a zipkin exporter
    let zipkin_exporter = opentelemetry_zipkin::new_pipeline()
        .with_service_name("trace-demo")
        .init_exporter()?;

    let provider = TracerProvider::builder()
        // We can build a span processor and pass it into provider.
        .with_span_processor(jaeger_processor)
        // For batch span processor, we can also provide the exporter and runtime and use this
        // helper function to build a batch span processor
        .with_batch_exporter(zipkin_exporter, opentelemetry_sdk::runtime::Tokio)
        // Same helper function is also available to build a simple span processor.
        .with_simple_exporter(StdoutExporter::default())
        .build();

    let _ = global::set_tracer_provider(provider);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    init_tracer()?;

    let tracer = global::tracer("jaeger-and-zipkin");

    {
        let span = tracer.start("first span");
        let _guard = mark_span_as_active(span);
        {
            let _inner = tracer.start("first sub span");
            tokio::time::sleep(Duration::from_millis(15)).await;
        }
        {
            let _inner = tracer.start("second sub span");
            tokio::time::sleep(Duration::from_millis(15)).await;
        }
    }

    tokio::time::sleep(Duration::from_millis(15)).await;

    shutdown_tracer_provider();

    Ok(())
}
