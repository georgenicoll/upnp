use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedMessage {
    pub start_line: String,
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyMessage,
    MissingStartLine,
    InvalidHeaderLine(String),
}

fn is_valid_header_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    name.bytes().all(|b| {
        b.is_ascii_alphanumeric()
            || matches!(
                b,
                b'!' | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'.'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'|'
                    | b'~'
            )
    })
}

pub fn parse_message(raw: &str) -> Result<ParsedMessage, ParseError> {
    if raw.trim().is_empty() {
        return Err(ParseError::EmptyMessage);
    }

    let normalized = raw.replace("\r\n", "\n");
    let mut lines = normalized.lines();

    let start_line = lines
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .ok_or(ParseError::MissingStartLine)?
        .to_string();

    let mut headers = BTreeMap::new();
    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let Some((name, value)) = line.split_once(':') else {
            return Err(ParseError::InvalidHeaderLine(line.to_string()));
        };

        let header_name = name.trim();
        if !is_valid_header_name(header_name) {
            return Err(ParseError::InvalidHeaderLine(line.to_string()));
        }

        headers.insert(header_name.to_ascii_uppercase(), value.trim().to_string());
    }

    Ok(ParsedMessage {
        start_line,
        headers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_m_search_message_smoke_test() {
        let input = "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\nST: ssdp:all\r\n\r\n";

        let parsed = parse_message(input).expect("valid message should parse");
        assert_eq!(parsed.start_line, "M-SEARCH * HTTP/1.1");
        assert_eq!(
            parsed.headers.get("HOST"),
            Some(&"239.255.255.250:1900".to_string())
        );
        assert_eq!(parsed.headers.get("MX"), Some(&"2".to_string()));
        assert_eq!(parsed.headers.get("ST"), Some(&"ssdp:all".to_string()));
    }

    #[test]
    fn parse_response_message_smoke_test() {
        let input = "HTTP/1.1 200 OK\r\nCACHE-CONTROL: max-age=120\r\nLOCATION: http://192.168.1.2:8080/device.xml\r\nST: upnp:rootdevice\r\nUSN: uuid:dummy::upnp:rootdevice\r\n\r\n";

        let parsed = parse_message(input).expect("valid response should parse");
        assert_eq!(parsed.start_line, "HTTP/1.1 200 OK");
        assert_eq!(
            parsed.headers.get("LOCATION"),
            Some(&"http://192.168.1.2:8080/device.xml".to_string())
        );
    }

    #[test]
    fn parse_fails_without_colon_in_header() {
        let input = "M-SEARCH * HTTP/1.1\r\nHOST 239.255.255.250:1900\r\n\r\n";
        let parsed = parse_message(input);

        assert!(matches!(
            parsed,
            Err(ParseError::InvalidHeaderLine(line)) if line == "HOST 239.255.255.250:1900"
        ));
    }
}
