//! HTTP security headers (ASVS L2 browser controls).

use axum::extract::{Request, State};
use axum::http::{header, HeaderName, HeaderValue};
use axum::middleware::Next;
use axum::response::Response;

use crate::state::AppState;

/// Attach security headers to every API response.
pub async fn security_headers_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    insert(headers, header::X_CONTENT_TYPE_OPTIONS, "nosniff");
    insert(
        headers,
        header::REFERRER_POLICY,
        "strict-origin-when-cross-origin",
    );
    insert(headers, HeaderName::from_static("x-frame-options"), "DENY");
    insert(
        headers,
        HeaderName::from_static("permissions-policy"),
        "camera=(), microphone=(), geolocation=()",
    );
    insert(
        headers,
        HeaderName::from_static("cross-origin-opener-policy"),
        "same-origin",
    );
    insert(
        headers,
        HeaderName::from_static("cross-origin-resource-policy"),
        "same-site",
    );
    // JSON API — deny active content; still set frame-ancestors.
    insert(
        headers,
        header::CONTENT_SECURITY_POLICY,
        "default-src 'none'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'",
    );
    insert(
        headers,
        HeaderName::from_static("x-permitted-cross-domain-policies"),
        "none",
    );
    insert(
        headers,
        HeaderName::from_static("cache-control"),
        "no-store",
    );

    if state.config.cookie_secure() {
        insert(
            headers,
            header::STRICT_TRANSPORT_SECURITY,
            "max-age=31536000; includeSubDomains",
        );
    }

    response
}

fn insert(headers: &mut axum::http::HeaderMap, name: HeaderName, value: &'static str) {
    if let Ok(value) = HeaderValue::from_str(value) {
        headers.insert(name, value);
    }
}
