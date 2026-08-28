//! The limit exists to protect the journal from being hammered. It must not
//! also throttle the static files the browser needs to render the journal:
//! one cold load of the client asks for a dozen or more hashed assets at once,
//! and counting those against the same budget makes a hard refresh able to
//! starve the app of its own code.

use std::net::SocketAddr;

use axum::Router;
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use config::RateLimitConfig;
use http_axum::router::rate_limited;
use tower::util::ServiceExt;

const CALLER: SocketAddr =
    SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST), 9000);

/// One request allowed, then nothing until it replenishes — so a second call
/// is refused and the difference between guarded and exempt is unmistakable.
fn strictest() -> RateLimitConfig {
    RateLimitConfig {
        enabled: true,
        requests_per_second: 1,
        burst: 1,
        trust_forwarded_for: false,
    }
}

fn under_test(config: &RateLimitConfig) -> Router {
    rate_limited(
        Router::new().route("/api/v1/entries", get(|| async { "journal" })),
        config,
    )
    .fallback(|| async { "index.html" })
}

async fn status_of(app: &Router, path: &str) -> StatusCode {
    let mut request = Request::builder().uri(path).body(Body::empty()).unwrap();
    request.extensions_mut().insert(ConnectInfo(CALLER));

    app.clone().oneshot(request).await.unwrap().status()
}

#[tokio::test]
async fn a_guarded_route_is_refused_once_its_budget_is_spent() {
    let app = under_test(&strictest());

    assert_eq!(status_of(&app, "/api/v1/entries").await, StatusCode::OK);
    assert_eq!(
        status_of(&app, "/api/v1/entries").await,
        StatusCode::TOO_MANY_REQUESTS
    );
}

#[tokio::test]
async fn client_assets_keep_being_served_after_the_budget_is_spent() {
    let app = under_test(&strictest());

    // Spend the whole budget on the API.
    assert_eq!(status_of(&app, "/api/v1/entries").await, StatusCode::OK);
    assert_eq!(
        status_of(&app, "/api/v1/entries").await,
        StatusCode::TOO_MANY_REQUESTS
    );

    // The files the client is made of are still served, as many as it asks for.
    for _ in 0..25 {
        assert_eq!(
            status_of(&app, "/assets/index-abc123.js").await,
            StatusCode::OK
        );
    }
}

#[tokio::test]
async fn a_switched_off_limit_guards_nothing() {
    let off = RateLimitConfig {
        enabled: false,
        ..strictest()
    };
    let app = under_test(&off);

    for _ in 0..10 {
        assert_eq!(status_of(&app, "/api/v1/entries").await, StatusCode::OK);
    }
}

/// `GovernorConfigBuilder::per_second` takes an interval, not a rate, so the
/// configured figure has to be converted or it means its own opposite.
mod replenishment {
    use http_axum::router::replenish_interval_millis;

    #[test]
    fn a_rate_becomes_the_interval_between_replenishments() {
        assert_eq!(replenish_interval_millis(1), 1000);
        assert_eq!(replenish_interval_millis(10), 100);
        assert_eq!(replenish_interval_millis(15), 66);
    }

    #[test]
    fn a_faster_rate_waits_a_shorter_time() {
        assert!(replenish_interval_millis(50) < replenish_interval_millis(5));
    }

    #[test]
    fn an_interval_is_never_zero_however_high_the_rate() {
        // The builder rejects a zero interval outright.
        assert_eq!(replenish_interval_millis(10_000), 1);
        assert_eq!(replenish_interval_millis(u64::MAX), 1);
    }

    #[test]
    fn a_nonsensical_rate_of_zero_falls_back_to_one_a_second() {
        assert_eq!(replenish_interval_millis(0), 1000);
    }
}
