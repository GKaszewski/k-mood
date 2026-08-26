mod client;
mod credential;
mod response;

pub use client::{SubsonicNowPlayingAdapter, auth_token};
pub use credential::SubsonicCredential;
pub use response::parse_now_playing;
