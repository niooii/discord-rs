use thiserror::Error;
#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("Custom: {text}")]
    Custom { text: String },
    #[error("DeserializeError: {err}")]
    DeserializeError{ err: String },
    #[error("UnwantedEventError: {event_name}")]
    UnwantedEventError{ event_name: String }
}