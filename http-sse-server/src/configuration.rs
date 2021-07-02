use tracing_subscriber::EnvFilter;
use tracing::log::Level::Warn;

pub fn configure_logging(log_level:&str){
    let filter = EnvFilter::from(log_level)
        .add_directive(format!("actix_server={}",Warn).parse().unwrap());

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(filter)
        .with_current_span(false)
        .init();
}