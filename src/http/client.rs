use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn fetch_http_resource(url: &str) -> io::Result<String> {
    let (host, port, path) = parse_http_url(url)?;
    let mut stream = TcpStream::connect((host.as_str(), port))?;

    let request =
        format!("GET {path} HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let (status_line, body) = split_http_response(&response)?;
    if !status_line.starts_with("HTTP/1.1 200") {
        return Err(io::Error::other(format!(
            "unexpected HTTP response: {status_line}"
        )));
    }

    Ok(body.to_string())
}

pub fn parse_http_url(url: &str) -> io::Result<(String, u16, String)> {
    let Some(rest) = url.strip_prefix("http://") else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only http:// URLs are supported",
        ));
    };

    let (authority, path) = match rest.split_once('/') {
        Some((authority, path)) => (authority, format!("/{path}")),
        None => (rest, "/".to_string()),
    };

    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port_text)) => {
            let port = port_text
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid port in URL"))?;
            (host.to_string(), port)
        }
        None => (authority.to_string(), 80),
    };

    Ok((host, port, path))
}

fn split_http_response(response: &str) -> io::Result<(&str, &str)> {
    let Some((headers, body)) = response.split_once("\r\n\r\n") else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "HTTP response was missing a body separator",
        ));
    };

    let mut lines = headers.lines();
    let Some(status_line) = lines.next() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "HTTP response was missing a status line",
        ));
    };

    Ok((status_line, body))
}
