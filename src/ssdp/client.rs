use std::io;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::time::Duration;

use crate::net::udp::UdpTransport;
use crate::ssdp::message::build_m_search;
use crate::ssdp::parser::{ParsedMessage, parse_message};

#[derive(Debug, Clone)]
pub struct DiscoveryResponse {
    pub peer: SocketAddr,
    pub message: ParsedMessage,
}

pub fn discover_once(
    bind_addr: SocketAddr,
    target_addr: SocketAddr,
    host_header: &str,
    search_target: &str,
    mx: u64,
    timeout: Duration,
) -> io::Result<Vec<DiscoveryResponse>> {
    let transport = UdpTransport::bind(bind_addr, "ssdp-client")?;
    transport.set_read_timeout(Some(timeout))?;

    let request = build_m_search(host_header, search_target, mx);
    let _ = transport.send_to(request.as_bytes(), target_addr)?;

    let mut responses = Vec::new();
    let mut buffer = [0u8; 4096];

    loop {
        match transport.recv_from(&mut buffer) {
            Ok((bytes, peer)) => {
                let payload = String::from_utf8_lossy(&buffer[..bytes]);
                if let Ok(parsed) = parse_message(&payload)
                    && parsed.start_line.starts_with("HTTP/1.1 200")
                    && parsed.headers.contains_key("LOCATION")
                    && parsed.headers.contains_key("ST")
                    && parsed.headers.contains_key("USN")
                {
                    responses.push(DiscoveryResponse {
                        peer,
                        message: parsed,
                    });
                }
            }
            Err(err)
                if err.kind() == ErrorKind::TimedOut || err.kind() == ErrorKind::WouldBlock =>
            {
                break;
            }
            Err(err) => return Err(err),
        }
    }

    Ok(responses)
}
