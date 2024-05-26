//! backend/src/telemetry.rs
//! To house logic and data regarding application telemetry (logging)
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Allows application to compose multiple layers into the tracing subscriber.
/// returning implementation because actual returned type is very complex.
pub fn get_subscriber<T>(
    name: String,
    env_filter_level: String,
    sink: T, // where to send logs (e.g. standard out)
) -> impl Subscriber + Send + Sync
where
    T: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter: EnvFilter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter_level));
    let formatting_layer: BunyanFormattingLayer<T> = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        // Adding tracing formatting
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Function shoud only be called **ONCE**!
/// This registers a trace subscriber as _global_ default to process
/// application span data.
pub fn init_subscriber<T>(subscriber: T)
where
    T: Subscriber + Send + Sync,
{
    LogTracer::init().expect("Failed to set Global Logger");
    set_global_default(subscriber).expect("Failed to set Global Subscriber")
}
