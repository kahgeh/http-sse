use tracing_subscriber::{EnvFilter, Registry};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_actix_web::{RootSpanBuilder, DefaultRootSpanBuilder};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{ Error};
use tracing::Span;
const HTTP_HEADER_TO_LOG: &str="internal-correlation-id";

pub struct HttpAppRootSpanBuilder {
}

impl RootSpanBuilder for HttpAppRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let header_value=match request.headers().get(HTTP_HEADER_TO_LOG){
            Some(header)=>header.to_str().unwrap(),
            _=>"none"
        };
        tracing_actix_web::root_span!(
            request,
            correlation_id=header_value)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

pub fn configure_logging(app_name: &str, log_level:&str){
     let filter = EnvFilter::from(log_level);

    let formatting_layer = BunyanFormattingLayer::new(
        app_name.into(),
        std::io::stdout);

    let subscriber = Registry::default()
        .with(filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}
