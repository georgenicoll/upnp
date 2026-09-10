use std::net::SocketAddr;
use std::sync::OnceLock;

use tracing::Level;
use tracing_subscriber::EnvFilter;

static LOGGING_INIT: OnceLock<()> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub enum PacketDirection {
    Inbound,
    Outbound,
}

#[derive(Debug)]
pub struct PacketLogEvent<'a> {
    pub component: &'a str,
    pub direction: PacketDirection,
    pub peer: SocketAddr,
    pub bytes: usize,
    pub preview: &'a str,
}

pub fn init_logging() {
    LOGGING_INIT.get_or_init(|| {
        let env_filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info,monkeynuthead_upnp=debug"))
            .expect("default log filter must be valid");

        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(true)
            .with_level(true)
            .with_thread_names(false)
            .with_line_number(false)
            .compact()
            .init();
    });
}

pub fn log_packet_event(event: &PacketLogEvent<'_>) {
    let direction = match event.direction {
        PacketDirection::Inbound => "in",
        PacketDirection::Outbound => "out",
    };

    tracing::event!(
        Level::DEBUG,
        component = event.component,
        direction,
        peer = %event.peer,
        bytes = event.bytes,
        preview = event.preview,
        "udp_packet"
    );
}

pub fn payload_preview(payload: &[u8], max_bytes: usize) -> String {
    let normalized = String::from_utf8_lossy(payload)
        .replace('\r', "\\r")
        .replace('\n', "\\n");

    let mut preview: String = normalized.chars().take(max_bytes).collect();
    if normalized.chars().count() > max_bytes {
        preview.push_str("...");
    }

    preview
}
