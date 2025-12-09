use std::time::Duration;

use http::HeaderMap;
use opentelemetry::{
    global,
    trace::{Span, Tracer},
    KeyValue,
    Context,
};
use opentelemetry::trace::TracerProvider; 
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    resource::Resource,
    trace::SdkTracerProvider,
};

use tokio::time::sleep;
use tracing::{info, span, Level};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tracing_subscriber::layer::Layer; 


fn init_tracing() -> SdkTracerProvider {
    let exporter = SpanExporter::builder()
        .with_http()
        .build()
        .expect("OTLP HTTP exporter");

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder()
                .with_service_name("jaeger-http-demo")
                .with_attributes(vec![
                    KeyValue::new("service.namespace", "demo"),
                    KeyValue::new("service.instance.id", "instance-1"),
                ])
                .build(),
        )
        .build();

    global::set_tracer_provider(provider.clone());
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = provider.tracer("jaeger-http-demo");

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_filter(EnvFilter::new("info")),
        )
        .init();

    provider
}

async fn frontend_request() {
    let root_span = span!(Level::INFO, "frontend", user_id = 42);
    let _enter = root_span.enter();

    info!("frontend: received request");

    let mut headers = HeaderMap::new();
    let cx = root_span.context();
    inject_trace_context(&cx, &mut headers);

    ads_service(headers).await;
}

async fn ads_service(headers: HeaderMap) {
    let ads = span!(Level::INFO, "ads-service");
    let _e = ads.enter();

    let remote_ctx = extract_context(&headers);
    ads.set_parent(remote_ctx);

    info!("ads-service: processing ads data");

    sleep(Duration::from_millis(100)).await;
}

fn inject_trace_context(cx: &Context, headers: &mut HeaderMap) {
    global::get_text_map_propagator(|prop| {
        prop.inject_context(cx, &mut opentelemetry_http::HeaderInjector(headers))
    });
}

fn extract_context(headers: &HeaderMap) -> Context {
    global::get_text_map_propagator(|prop| {
        prop.extract(&opentelemetry_http::HeaderExtractor(headers))
    })
}

#[tokio::main]
async fn main() {
    let provider = init_tracing();

    frontend_request().await;

    provider.shutdown().expect("shutdown");
}
