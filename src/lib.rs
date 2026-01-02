//! Communication protocols, message routing, and ACL for multi-agent systems.

pub mod message;
pub mod builder;
pub mod performative;
pub mod router;
pub mod mailbox;
pub mod error;
pub mod protocols;
pub mod prelude;

// Re-exports
pub use message::{Message, MessageId};
pub use builder::MessageBuilder;
pub use performative::Performative;
pub use router::Router;
pub use mailbox::Mailbox;
pub use error::MessagingError;
