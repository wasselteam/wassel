use std::fmt::Display;
use std::io::Write as _;
use std::time::Duration;

use bytes::Bytes;
use serde::Serialize;

#[derive(Default)]
pub struct SseMessage {
    pub event: Option<String>,
    pub data: String,
    pub id: Option<i64>,
    pub retry: Option<Duration>,
}

impl SseMessage {
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: data.into(),
            ..Default::default()
        }
    }

    pub fn new_json(value: impl Serialize) -> Result<Self, serde_json::Error> {
        let data = serde_json::to_string(&value)?;
        Ok(Self {
            data,
            ..Default::default()
        })
    }

    pub fn with_event(self, event: impl Into<String>) -> Self {
        Self {
            event: Some(event.into()),
            ..self
        }
    }

    pub fn with_id(self, id: i64) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn with_retry(self, retry: Duration) -> Self {
        Self {
            retry: Some(retry),
            ..self
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let _ = write!(bytes, "{self}");
        bytes
    }
}

impl Display for SseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(event) = &self.event {
            writeln!(f, "event: {event}")?;
        }

        if let Some(id) = self.id {
            writeln!(f, "id: {id}")?;
        }

        if let Some(retry) = self.retry {
            let millis = retry.as_millis();
            writeln!(f, "retry: {millis}")?;
        }

        for line in self.data.split('\n') {
            writeln!(f, "data: {line}")?;
        }

        writeln!(f)?;

        Ok(())
    }
}

impl From<SseMessage> for Bytes {
    fn from(value: SseMessage) -> Self {
        Bytes::from(value.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_sse_message() {
        let msg = SseMessage::new("Hello, World!");
        assert_eq!(msg.to_string(), "data: Hello, World!\n\n");
    }

    #[test]
    fn test_sse_message_multiline() {
        let msg = SseMessage::new("Hello, World!\nSecond Line\nThird Line");
        assert_eq!(
            msg.to_string(),
            "data: Hello, World!\ndata: Second Line\ndata: Third Line\n\n"
        );
    }

    #[test]
    fn test_sse_message_with_event() {
        let msg = SseMessage::new("Hello, World!").with_event("greet");
        assert_eq!(msg.to_string(), "event: greet\ndata: Hello, World!\n\n");
    }

    #[test]
    fn test_sse_message_with_id() {
        let msg = SseMessage::new("Hello, World!").with_id(2281337);
        assert_eq!(msg.to_string(), "id: 2281337\ndata: Hello, World!\n\n");
    }

    #[test]
    fn test_sse_message_with_retry() {
        let msg = SseMessage::new("Hello, World!").with_retry(Duration::from_secs(5));
        assert_eq!(msg.to_string(), "retry: 5000\ndata: Hello, World!\n\n");
    }

    #[test]
    fn test_sse_message_with_full() {
        let msg = SseMessage::new("Hello, World!")
            .with_event("greet")
            .with_id(2281337)
            .with_retry(Duration::from_secs(5));
        assert_eq!(
            msg.to_string(),
            "event: greet\nid: 2281337\nretry: 5000\ndata: Hello, World!\n\n"
        );
    }

    #[test]
    fn test_sse_message_with_bytes() {
        let msg = SseMessage::new("Hello, World!")
            .with_event("greet")
            .with_id(2281337)
            .with_retry(Duration::from_secs(5));
        let bytes: Bytes = msg.into();
        assert_eq!(
            bytes,
            Bytes::from_static(b"event: greet\nid: 2281337\nretry: 5000\ndata: Hello, World!\n\n")
        );
    }
}
