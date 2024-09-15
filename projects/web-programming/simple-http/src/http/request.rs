use super::internal::{Method, Path, HttpHeader, HttpVersion};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,          // HTTP Method (GET, POST, etc.)
    pub path: Path,              // Request Path (with optional query string)
    pub headers: HttpHeader,     // HTTP Headers
    pub body: String,            // Request body (if any)
    pub http_version: HttpVersion,   // HTTP Version (e.g., HTTP/1.1)
}

impl HttpRequest {
    // Parse a raw HTTP request buffer into an HttpRequest struct
    pub fn from_buffer(buffer: &[u8]) -> HttpRequest {
        let request = String::from_utf8_lossy(buffer);
        let mut lines = request.lines();
        
        // Parse the request line (e.g., "GET /index.html HTTP/1.1")
        let request_line = lines.next().unwrap();
        let mut parts = request_line.split_whitespace();
        
        let method = match parts.next().unwrap() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            _ => Method::GET, // Default method if not recognized
        };
        
        let path = Path::from_str(parts.next().unwrap());
        let version = HttpVersion::from_str(parts.next().unwrap());

        // Collect all headers
        let header_lines: Vec<&str> = lines.clone().take_while(|line| !line.is_empty()).collect();
        let headers = HttpHeader::from_lines(&header_lines);

        // Collect the body (if any), after the headers
        let body = lines.skip_while(|line| !line.is_empty()).skip(1).collect::<Vec<&str>>().join("\n");

        HttpRequest {
            method,
            path,
            headers,
            body,
            http_version: version,
        }
    }
}

    
