pub const CONTENT_TYPE_VALUE: &str = "text/event-stream";
pub const LAST_EVENT_ID_HEADER_KEY: &str = "Last-Event-ID";

//
pub mod event;

pub use event::Event;

//
#[cfg(feature = "stream")]
pub mod stream;

//
pub mod utils;
