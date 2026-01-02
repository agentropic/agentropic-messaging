use thiserror::Error;

/// Messaging errors
#[derive(Error, Debug)]
pub enum MessagingError {
    /// Agent not found
    #[error("Agent not found")]
    AgentNotFound,
    
    /// Send failed
    #[error("Send failed: {0}")]
    SendFailed(String),
    
    /// Lock error
    #[error("Lock error")]
    LockError,
    
    /// Other error
    #[error("Messaging error: {0}")]
    Other(String),
}
