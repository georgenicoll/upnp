use std::process::ExitCode;
use std::time::Duration;

use monkeynuthead_upnp::logging::init_logging;
use monkeynuthead_upnp::ssdp::client::discover_once;
use monkeynuthead_upnp::ssdp::constants::{
    SSDP_MULTICAST_SOCKET, SSDP_MX_MAX_SECS, SSDP_READ_TIMEOUT_MS,
};

fn main() -> ExitCode {
    init_logging();

    let target_socket =
        std::env::var("SSDP_TARGET").unwrap_or_else(|_| SSDP_MULTICAST_SOCKET.to_string());
    let bind_socket = std::env::var("SSDP_BIND").unwrap_or_else(|_| "0.0.0.0:0".to_string());
    let search_target = std::env::var("SSDP_ST").unwrap_or_else(|_| "ssdp:all".to_string());

    let target_addr = target_socket
        .parse()
        .expect("SSDP_TARGET should be a valid socket address");
    let bind_addr = bind_socket
        .parse()
        .expect("SSDP_BIND should be a valid socket address");

    let responses = match discover_once(
        bind_addr,
        target_addr,
        &target_socket,
        &search_target,
        SSDP_MX_MAX_SECS,
        Duration::from_millis(SSDP_READ_TIMEOUT_MS),
    ) {
        Ok(responses) => responses,
        Err(err) => {
            tracing::error!(error = %err, "discovery failed");
            return ExitCode::from(1);
        }
    };

    tracing::info!(count = responses.len(), "discovery complete");
    for response in responses {
        let st = response
            .message
            .headers
            .get("ST")
            .map_or("<missing>", String::as_str);
        let usn = response
            .message
            .headers
            .get("USN")
            .map_or("<missing>", String::as_str);
        let location = response
            .message
            .headers
            .get("LOCATION")
            .map_or("<missing>", String::as_str);

        println!(
            "peer={} st={} usn={} location={}",
            response.peer, st, usn, location
        );
    }

    ExitCode::SUCCESS
}
