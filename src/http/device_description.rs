use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};

#[derive(Debug, Clone)]
pub struct DeviceDescriptionConfig {
    pub bind_addr: SocketAddr,
    pub friendly_name: String,
    pub device_type: String,
    pub udn: String,
    pub manufacturer: String,
    pub model_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptionFields {
    pub friendly_name: String,
    pub device_type: String,
    pub udn: String,
    pub manufacturer: String,
    pub model_name: String,
}

pub fn build_device_description_xml(config: &DeviceDescriptionConfig) -> String {
    format!(
        r#"<?xml version="1.0"?>
<root xmlns="urn:schemas-upnp-org:device-1-0">
  <specVersion>
    <major>1</major>
    <minor>0</minor>
  </specVersion>
  <device>
    <deviceType>{}</deviceType>
    <friendlyName>{}</friendlyName>
    <manufacturer>{}</manufacturer>
    <modelName>{}</modelName>
    <UDN>{}</UDN>
  </device>
</root>
"#,
        escape_xml(&config.device_type),
        escape_xml(&config.friendly_name),
        escape_xml(&config.manufacturer),
        escape_xml(&config.model_name),
        escape_xml(&config.udn),
    )
}

pub fn parse_device_description_fields(xml: &str) -> io::Result<DeviceDescriptionFields> {
    Ok(DeviceDescriptionFields {
        device_type: extract_tag(xml, "deviceType")?,
        friendly_name: extract_tag(xml, "friendlyName")?,
        manufacturer: extract_tag(xml, "manufacturer")?,
        model_name: extract_tag(xml, "modelName")?,
        udn: extract_tag(xml, "UDN")?,
    })
}

pub fn serve_once(config: &DeviceDescriptionConfig) -> io::Result<()> {
    let listener = TcpListener::bind(config.bind_addr)?;
    let (mut stream, _) = listener.accept()?;

    let mut request = [0u8; 2048];
    let _ = stream.read(&mut request)?;

    let body = build_device_description_xml(config);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/xml; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn extract_tag(xml: &str, tag: &str) -> io::Result<String> {
    let start_token = format!("<{tag}>");
    let end_token = format!("</{tag}>");
    let start = xml.find(&start_token).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("missing <{tag}> element"),
        )
    })? + start_token.len();
    let remainder = &xml[start..];
    let end = remainder.find(&end_token).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("missing </{tag}> element"),
        )
    })?;
    Ok(remainder[..end].to_string())
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::http::client::fetch_http_resource;

    fn reserve_ephemeral_port() -> io::Result<u16> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        drop(listener);
        Ok(port)
    }

    #[test]
    fn http_server_serves_and_client_parses_device_description() {
        let port = reserve_ephemeral_port().expect("port should be reserved");
        let bind_addr: SocketAddr = format!("127.0.0.1:{port}")
            .parse()
            .expect("valid bind address");

        let config = DeviceDescriptionConfig {
            bind_addr,
            friendly_name: "monkeynuthead-upnp demo device".to_string(),
            device_type: "urn:schemas-upnp-org:device:MediaServer:1".to_string(),
            udn: "uuid:demo-device".to_string(),
            manufacturer: "monkeynuthead".to_string(),
            model_name: "upnp-demo".to_string(),
        };

        let server_cfg = config.clone();
        let server = thread::spawn(move || serve_once(&server_cfg));

        thread::sleep(Duration::from_millis(30));

        let location = format!("http://127.0.0.1:{port}/device.xml");
        let xml = fetch_http_resource(&location).expect("device description should be fetched");
        let fields =
            parse_device_description_fields(&xml).expect("device description should parse");

        server
            .join()
            .expect("server thread should not panic")
            .expect("server should not fail");

        assert_eq!(fields.friendly_name, config.friendly_name);
        assert_eq!(fields.device_type, config.device_type);
        assert_eq!(fields.udn, config.udn);
        assert_eq!(fields.manufacturer, config.manufacturer);
        assert_eq!(fields.model_name, config.model_name);
    }
}
