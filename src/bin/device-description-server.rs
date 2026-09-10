use std::process::ExitCode;

use monkeynuthead_upnp::http::device_description::{DeviceDescriptionConfig, serve_once};
use monkeynuthead_upnp::logging::init_logging;

fn main() -> ExitCode {
    init_logging();

    let bind_addr = std::env::var("HTTP_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8000".to_string())
        .parse()
        .expect("HTTP_BIND should be a valid socket address");

    let config = DeviceDescriptionConfig {
        bind_addr,
        friendly_name: std::env::var("DEVICE_FRIENDLY_NAME")
            .unwrap_or_else(|_| "monkeynuthead-upnp demo device".to_string()),
        device_type: std::env::var("DEVICE_TYPE")
            .unwrap_or_else(|_| "urn:schemas-upnp-org:device:MediaServer:1".to_string()),
        udn: std::env::var("DEVICE_UDN")
            .unwrap_or_else(|_| "uuid:monkeynuthead-upnp-demo".to_string()),
        manufacturer: std::env::var("DEVICE_MANUFACTURER")
            .unwrap_or_else(|_| "monkeynuthead".to_string()),
        model_name: std::env::var("DEVICE_MODEL").unwrap_or_else(|_| "upnp-demo".to_string()),
    };

    tracing::info!(%bind_addr, "waiting for one HTTP device description request");

    match serve_once(&config) {
        Ok(()) => {
            tracing::info!("served one device description response");
            ExitCode::SUCCESS
        }
        Err(err) => {
            tracing::error!(error = %err, "device description server failed");
            ExitCode::from(1)
        }
    }
}
