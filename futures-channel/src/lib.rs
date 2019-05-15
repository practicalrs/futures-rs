//! Asynchronous channels.
//!
//! This crate provides channels that can be used to communicate between
//! asynchronous tasks.
//!
//! All items of this library are only available when the `std` feature of this
//! library is activated, and it is activated by default.

#![cfg_attr(not(feature = "std"), no_std)]

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![warn(clippy::all)]

#![doc(html_root_url = "https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures_channel")]

#[cfg(feature = "std")]
mod lock;
#[cfg(feature = "std")]
pub mod mpsc;
#[cfg(feature = "std")]
pub mod oneshot;
