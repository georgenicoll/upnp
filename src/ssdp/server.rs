use std::io;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::net::udp::UdpTransport;
use crate::ssdp::message::build_discovery_response;
use crate::ssdp::parser::parse_message;

#[derive(Debug, Clone)]
pub struct ResponderConfig {
    pub bind_addr: SocketAddr,
    pub location: String,
    pub search_target: String,
    pub usn: String,
    pub max_age: u64,
    pub read_timeout: Option<Duration>,
}

fn should_reply(search_target: &str, expected_target: &str) -> bool {
    search_target.eq_ignore_ascii_case("ssdp:all")
        || search_target.eq_ignore_ascii_case(expected_target)
}

pub fn respond_once(config: &ResponderConfig) -> io::Result<bool> {
    respond_once_inner(config, None)
}

fn respond_once_inner(config: &ResponderConfig, ready: Option<Sender<()>>) -> io::Result<bool> {
    let transport = UdpTransport::bind(config.bind_addr, "ssdp-server")?;
    transport.set_read_timeout(config.read_timeout)?;

    if let Some(ready) = ready {
        let _ = ready.send(());
    }

    let mut buffer = [0u8; 4096];
    let (bytes, peer) = transport.recv_from(&mut buffer)?;

    let payload = String::from_utf8_lossy(&buffer[..bytes]);
    let parsed = match parse_message(&payload) {
        Ok(parsed) => parsed,
        Err(_) => return Ok(false),
    };

    let is_m_search = parsed.start_line.starts_with("M-SEARCH ");
    let Some(man) = parsed.headers.get("MAN") else {
        return Ok(false);
    };
    let Some(st) = parsed.headers.get("ST") else {
        return Ok(false);
    };

    if !is_m_search || !man.contains("ssdp:discover") || !should_reply(st, &config.search_target) {
        return Ok(false);
    }

    let response = build_discovery_response(
        &config.location,
        &config.search_target,
        &config.usn,
        config.max_age,
    );
    let _ = transport.send_to(response.as_bytes(), peer)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, UdpSocket};
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::ssdp::client::discover_once;

    fn reserve_ephemeral_port() -> io::Result<u16> {
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        let port = socket.local_addr()?.port();
        drop(socket);
        Ok(port)
    }

    #[test]
    fn client_discovers_local_responder() {
        let port = reserve_ephemeral_port().expect("port should be reserved");
        let bind_addr: SocketAddr = format!("127.0.0.1:{port}")
            .parse()
            .expect("valid bind address");

        let config = ResponderConfig {
            bind_addr,
            location: "http://127.0.0.1:8000/device.xml".to_string(),
            search_target: "upnp:rootdevice".to_string(),
            usn: "uuid:dummy-device::upnp:rootdevice".to_string(),
            max_age: 1800,
            read_timeout: Some(Duration::from_millis(500)),
        };

        let server_cfg = config.clone();
        let (ready_tx, ready_rx) = channel();
        let server = thread::spawn(move || respond_once_inner(&server_cfg, Some(ready_tx)));

        ready_rx.recv().expect("server should signal readiness");

        let responses = discover_once(
            "127.0.0.1:0".parse().expect("valid client bind address"),
            bind_addr,
            &format!("127.0.0.1:{port}"),
            "upnp:rootdevice",
            2,
            Duration::from_millis(250),
        )
        .expect("discovery should complete");

        let served = server
            .join()
            .expect("server thread should not panic")
            .expect("server should not fail");

        assert!(served, "server should have responded to valid M-SEARCH");
        assert_eq!(responses.len(), 1, "expected one discovery response");

        let response = &responses[0];
        assert_eq!(
            response.message.headers.get("ST"),
            Some(&"upnp:rootdevice".to_string())
        );
        assert_eq!(
            response.message.headers.get("USN"),
            Some(&"uuid:dummy-device::upnp:rootdevice".to_string())
        );
        assert_eq!(
            response.message.headers.get("LOCATION"),
            Some(&"http://127.0.0.1:8000/device.xml".to_string())
        );
    }
}
