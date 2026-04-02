use std::time::Duration;
use tonic::transport::{ClientTlsConfig, Endpoint, Error};

use super::gateway;

/// Configures the endpoint for the gRPC client.
///
/// Sets reasonable settings to handle timeouts and keep-alive.
/// When gateway is configured (via `SP1_GATEWAY_HOST` env var), the endpoint address
/// is rewritten to the gateway host while the original host is preserved for the
/// `third-host` header (injected by the gateway interceptor on each RPC call).
pub fn configure_endpoint(addr: &str) -> Result<Endpoint, Error> {
    let effective_addr = gateway::rewrite_url_for_gateway(addr);

    let mut endpoint = Endpoint::new(effective_addr.clone())?
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(15))
        .keep_alive_while_idle(true)
        .http2_keep_alive_interval(Duration::from_secs(15))
        .keep_alive_timeout(Duration::from_secs(15))
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .tcp_nodelay(true);

    // Configure TLS if using HTTPS.
    if effective_addr.starts_with("https://") {
        #[cfg(target_os = "ios")]
        let tls_config = ClientTlsConfig::new().with_webpki_roots();
        #[cfg(not(target_os = "ios"))]
        let tls_config = ClientTlsConfig::new().with_enabled_roots();
        endpoint = endpoint.tls_config(tls_config)?;
    }

    Ok(endpoint)
}
