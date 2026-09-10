use std::process::ExitCode;
use std::time::Duration;

use monkeynuthead_upnp::logging::init_logging;
use monkeynuthead_upnp::ssdp::constants::SSDP_PORT;
use monkeynuthead_upnp::ssdp::server::{ResponderConfig, respond_once};

fn main() -> ExitCode {
    init_logging();

    let bind_addr = std::env::var("SSDP_BIND")
        .unwrap_or_else(|_| format!("0.0.0.0:{SSDP_PORT}"))
        .parse()
        .expect("SSDP_BIND should be a valid socket address");

    let location = std::env::var("SSDP_LOCATION")
        .unwrap_or_else(|_| "http://127.0.0.1:8000/device.xml".to_string());
    let search_target = std::env::var("SSDP_ST").unwrap_or_else(|_| "upnp:rootdevice".to_string());
    let usn = std::env::var("SSDP_USN")
        .unwrap_or_else(|_| "uuid:monkeynuthead-device::upnp:rootdevice".to_string());

    let config = ResponderConfig {
        bind_addr,
        location,
        search_target,
        usn,
        max_age: 1800,
        read_timeout: Some(Duration::from_secs(30)),
    };

    tracing::info!(%bind_addr, "waiting for one SSDP M-SEARCH request");

    match respond_once(&config) {
        Ok(true) => {
            tracing::info!("responded to one discovery request");
            ExitCode::SUCCESS
        }
        Ok(false) => {
            tracing::warn!("received packet did not match expected M-SEARCH format");
            ExitCode::from(1)
        }
        Err(err) => {
            tracing::error!(error = %err, "responder failed");
            ExitCode::from(1)
        }
    }
}
