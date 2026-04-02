//! # Gateway Proxy Support
//!
//! When the environment cannot directly reach the SP1 prover network, all gRPC
//! and HTTP requests can be routed through an API gateway (e.g. APISIX).
//!
//! ## How it works
//!
//! The gateway acts as a reverse-proxy.  The caller replaces the original
//! destination host with the gateway host and passes the original host in a
//! `third-host` header so the gateway can forward the request to the correct
//! upstream.
//!
//! ## Environment variables
//!
//! | Variable                      | Required | Description                              |
//! |-------------------------------|----------|------------------------------------------|
//! | `SP1_GATEWAY_HOST`            | yes      | Gateway origin, e.g. `http://gw:9080`    |
//! | `SP1_GATEWAY_TOKEN`           | yes      | Value for the `third-token` header       |
//! | `SP1_GATEWAY_SOURCE_SERVICE`  | yes      | Value for the `source-service` header    |
//!
//! When `SP1_GATEWAY_HOST` is **not** set, every helper in this module is a
//! no-op and the SDK behaves exactly as before.

use std::sync::OnceLock;

use reqwest::header::{HeaderMap, HeaderValue};

/// Cached gateway configuration read once from the environment.
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    /// Gateway base URL (e.g. `http://apisix.swimlane.svc.base.local:9080`).
    /// Must be a pure origin — no trailing path.
    pub gateway_host: String,
    /// Value for the `third-token` header.
    pub token: String,
    /// Value for the `source-service` header.
    pub source_service: String,
}

/// Returns the gateway config if `SP1_GATEWAY_HOST` is set, `None` otherwise.
///
/// # Panics
///
/// Panics if `SP1_GATEWAY_HOST` is set but `SP1_GATEWAY_TOKEN` or
/// `SP1_GATEWAY_SOURCE_SERVICE` is missing or empty.
pub fn get_gateway_config() -> Option<&'static GatewayConfig> {
    static CONFIG: OnceLock<Option<GatewayConfig>> = OnceLock::new();
    CONFIG
        .get_or_init(|| {
            let gateway_host = std::env::var("SP1_GATEWAY_HOST").ok().filter(|s| !s.is_empty())?;

            let token = std::env::var("SP1_GATEWAY_TOKEN")
                .expect("SP1_GATEWAY_HOST is set but SP1_GATEWAY_TOKEN is missing");
            let source_service = std::env::var("SP1_GATEWAY_SOURCE_SERVICE")
                .expect("SP1_GATEWAY_HOST is set but SP1_GATEWAY_SOURCE_SERVICE is missing");

            tracing::info!(
                gateway_host = %gateway_host,
                source_service = %source_service,
                "SP1 gateway proxy enabled"
            );

            Some(GatewayConfig { gateway_host, token, source_service })
        })
        .as_ref()
}

// ---------------------------------------------------------------------------
// URL helpers — intentionally avoid the `url` crate to keep deps unchanged.
// ---------------------------------------------------------------------------

/// Extract the host (and optional port) from a URL string.
///
/// `"https://rpc.production.succinct.xyz:443/path"` → `"rpc.production.succinct.xyz:443"`
/// `"https://rpc.production.succinct.xyz/path"`     → `"rpc.production.succinct.xyz"`
fn extract_host(url: &str) -> String {
    // Strip scheme ("https://", "http://").
    let without_scheme = url
        .find("://")
        .map(|i| &url[i + 3..])
        .unwrap_or(url);
    // Take everything before the first '/' or '?' (the authority).
    let authority = without_scheme
        .find(|c: char| c == '/' || c == '?')
        .map(|i| &without_scheme[..i])
        .unwrap_or(without_scheme);
    // Strip userinfo if present.
    authority.rsplit_once('@').map(|(_, hp)| hp).unwrap_or(authority).to_string()
}

/// Extract the path + query + fragment from a URL string — everything after
/// the authority, returned verbatim so presigned URL signatures stay intact.
///
/// `"https://s3.amazonaws.com/obj?X-Amz-Signature=abc"` → `"/obj?X-Amz-Signature=abc"`
/// `"https://example.com"`                               → `""`
fn extract_path_and_query(url: &str) -> &str {
    let without_scheme = url
        .find("://")
        .map(|i| &url[i + 3..])
        .unwrap_or(url);
    without_scheme
        .find('/')
        .map(|i| &without_scheme[i..])
        .unwrap_or("")
}

/// If gateway is configured, replace the scheme + authority in `original_url`
/// with the gateway host while preserving the path, query string and fragment
/// byte-for-byte.  Returns the original URL unchanged when the gateway is not
/// configured.
pub fn rewrite_url_for_gateway(original_url: &str) -> String {
    let Some(cfg) = get_gateway_config() else {
        return original_url.to_string();
    };

    let suffix = extract_path_and_query(original_url);
    let gateway = cfg.gateway_host.trim_end_matches('/');
    let rewritten = format!("{gateway}{suffix}");

    tracing::debug!(
        original = %original_url,
        rewritten = %rewritten,
        "Rewrote URL for gateway"
    );

    rewritten
}

/// Build a [`HeaderMap`] with gateway-specific headers.
///
/// `original_url` is the URL before rewriting — its host is used as the
/// `third-host` value so the gateway knows the upstream destination.
///
/// Returns `None` when the gateway is not configured.
pub fn build_gateway_headers(original_url: &str) -> Option<HeaderMap> {
    let cfg = get_gateway_config()?;
    let third_host = extract_host(original_url);

    let mut headers = HeaderMap::new();
    if let Ok(v) = HeaderValue::from_str(&cfg.token) {
        headers.insert("third-token", v);
    }
    if let Ok(v) = HeaderValue::from_str(&third_host) {
        headers.insert("third-host", v);
    }
    if let Ok(v) = HeaderValue::from_str(&cfg.source_service) {
        headers.insert("source-service", v);
    }

    Some(headers)
}

// ---------------------------------------------------------------------------
// gRPC interceptor
// ---------------------------------------------------------------------------

/// Create a tonic `Interceptor` for gRPC calls.  When the gateway is not
/// configured the interceptor is a no-op pass-through, so callers never need
/// to branch on `Option`.
///
/// `original_rpc_url` is needed to derive the `third-host` value.
pub fn make_interceptor(original_rpc_url: &str) -> GatewayInterceptor {
    match get_gateway_config() {
        Some(cfg) => GatewayInterceptor {
            token: Some(cfg.token.clone()),
            third_host: Some(extract_host(original_rpc_url)),
            source_service: Some(cfg.source_service.clone()),
        },
        None => GatewayInterceptor {
            token: None,
            third_host: None,
            source_service: None,
        },
    }
}

/// A [`tonic::service::Interceptor`] that adds gateway headers to every gRPC
/// call.  When all fields are `None` (no gateway configured) the interceptor
/// is a zero-cost pass-through.
#[derive(Debug, Clone)]
pub struct GatewayInterceptor {
    token: Option<String>,
    third_host: Option<String>,
    source_service: Option<String>,
}

impl tonic::service::Interceptor for GatewayInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let meta = request.metadata_mut();
        if let Some(ref token) = self.token {
            if let Ok(v) = token.parse() {
                meta.insert("third-token", v);
            }
        }
        if let Some(ref host) = self.third_host {
            if let Ok(v) = host.parse() {
                meta.insert("third-host", v);
            }
        }
        if let Some(ref svc) = self.source_service {
            if let Ok(v) = svc.parse() {
                meta.insert("source-service", v);
            }
        }
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_host() {
        assert_eq!(extract_host("https://rpc.production.succinct.xyz/v1"), "rpc.production.succinct.xyz");
        assert_eq!(extract_host("https://rpc.production.succinct.xyz:443/v1"), "rpc.production.succinct.xyz:443");
        assert_eq!(extract_host("http://localhost:9080"), "localhost:9080");
        assert_eq!(extract_host("https://bucket.s3.amazonaws.com/key?sig=abc"), "bucket.s3.amazonaws.com");
    }

    #[test]
    fn test_extract_path_and_query() {
        assert_eq!(extract_path_and_query("https://example.com/path?q=1&s=2"), "/path?q=1&s=2");
        assert_eq!(extract_path_and_query("https://example.com"), "");
        assert_eq!(extract_path_and_query("https://s3.aws.com/obj?X-Amz-Signature=abc&Expires=3600"), "/obj?X-Amz-Signature=abc&Expires=3600");
    }
}
