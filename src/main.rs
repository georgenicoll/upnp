use monkeynuthead_upnp::logging::init_logging;

fn main() {
    init_logging();
    tracing::info!("monkeynuthead-upnp: milestone 3 HTTP description endpoint is ready");
    tracing::info!("run `cargo run --bin ssdp-server` to answer one M-SEARCH request");
    tracing::info!("run `cargo run --bin device-description-server` to serve the XML description");
    tracing::info!("run `cargo run --bin ssdp-client` to send one M-SEARCH and print responses");
    tracing::info!("for local loopback demo use SSDP_BIND/SSDP_TARGET and HTTP_BIND env vars");
}
