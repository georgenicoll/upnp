pub fn build_m_search(host: &str, search_target: &str, mx: u64) -> String {
    format!(
        "M-SEARCH * HTTP/1.1\r\nHOST: {host}\r\nMAN: \"ssdp:discover\"\r\nMX: {mx}\r\nST: {search_target}\r\n\r\n"
    )
}

pub fn build_discovery_response(
    location: &str,
    search_target: &str,
    usn: &str,
    max_age: u64,
) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nCACHE-CONTROL: max-age={max_age}\r\nEXT:\r\nLOCATION: {location}\r\nST: {search_target}\r\nUSN: {usn}\r\nSERVER: monkeynuthead-upnp/0.1 UPnP/1.1\r\n\r\n"
    )
}
