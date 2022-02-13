#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub const CONTENT_TYPE_VALUE: &str = "text/event-stream";
pub const LAST_EVENT_ID_HEADER_KEY: &str = "Last-Event-ID";

//
#[cfg(feature = "alloc")]
pub mod event;

#[cfg(feature = "alloc")]
pub use event::Event;

//
#[cfg(feature = "stream")]
pub mod stream;

//
pub mod utils;
