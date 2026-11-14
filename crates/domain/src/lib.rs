#![allow(clippy::module_inception)]

pub mod activity;
pub mod api_token;
pub mod attachment;
pub mod auth;
pub mod comparison;
pub mod correlation;
pub mod cycle;
pub mod dimension;
pub mod entry;
pub mod errors;
pub mod events;
pub mod job;
pub mod location;
pub mod metric;
pub mod moon;
pub mod ports;
pub mod provider;
pub mod push;
pub mod rejection;
pub mod reminder;
pub mod services;
pub mod song;
pub mod statistics;
pub mod trend;
pub mod user;
pub mod weather;

mod macros;

#[cfg(feature = "test-helpers")]
pub mod testing;
