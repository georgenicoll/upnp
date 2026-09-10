use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use crate::logging::{PacketDirection, PacketLogEvent, log_packet_event, payload_preview};

pub struct UdpTransport {
    socket: UdpSocket,
    component_name: String,
}

impl UdpTransport {
    pub fn bind(bind_addr: SocketAddr, component_name: impl Into<String>) -> io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr)?;
        Ok(Self {
            socket,
            component_name: component_name.into(),
        })
    }

    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.socket.set_read_timeout(timeout)
    }

    pub fn send_to(&self, payload: &[u8], peer: SocketAddr) -> io::Result<usize> {
        let bytes = self.socket.send_to(payload, peer)?;
        let preview = payload_preview(payload, 80);
        let event = PacketLogEvent {
            component: &self.component_name,
            direction: PacketDirection::Outbound,
            peer,
            bytes,
            preview: &preview,
        };
        log_packet_event(&event);

        Ok(bytes)
    }

    pub fn recv_from(&self, buffer: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        let (bytes, peer) = self.socket.recv_from(buffer)?;
        let preview = payload_preview(&buffer[..bytes], 80);
        let event = PacketLogEvent {
            component: &self.component_name,
            direction: PacketDirection::Inbound,
            peer,
            bytes,
            preview: &preview,
        };
        log_packet_event(&event);

        Ok((bytes, peer))
    }
}
