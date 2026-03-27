use core::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Local};
use tokio::sync::broadcast;
use tracing::{
    Event, Level, Subscriber,
    field::{Field, Visit},
    level_filters::LevelFilter,
};
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan, layer::Context, prelude::*};

#[derive(Clone)]
pub struct LogMessage {
    pub fields: Arc<HashMap<String, String>>,
    pub level: Level,
    pub timestamp: DateTime<Local>,
}

impl LogMessage {
    fn new(level: Level) -> Self {
        Self {
            fields: Arc::default(),
            level,
            timestamp: Local::now(),
        }
    }
}

impl Visit for LogMessage {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        Arc::get_mut(&mut self.fields)
            .unwrap()
            .insert(field.name().to_owned(), format!("{value:?}"));
    }
}

struct TracingSubscriberLayer {
    sender: broadcast::Sender<LogMessage>,
}

impl<S: Subscriber> tracing_subscriber::Layer<S> for TracingSubscriberLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut msg = LogMessage::new(*event.metadata().level());
        event.record(&mut msg);
        let _ = self.sender.send(msg);
    }
}

pub fn init_tracing_subscriber() -> broadcast::Receiver<LogMessage> {
    let (sender, receiver) = broadcast::channel(128);
    let layer = TracingSubscriberLayer { sender };

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("wasmtime=info".parse().expect("parsing tracing directive"))
        .add_directive(
            "cranelift_codegen=info"
                .parse()
                .expect("parsing tracing directive"),
        )
        .add_directive(
            "cranelift_frontend=info"
                .parse()
                .expect("parsing tracing directive"),
        );

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .finish()
        .with(layer)
        .init();

    receiver
}
