use std::collections::HashMap;

use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    // Add other HTTP methods as needed
}

impl Method {
    // Returns the string representation of the HTTP method
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
        }
    }
}

// Implement the FromStr trait to convert from a string to a Method enum
impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            _ => Err(()),  // Return an error if the method is not recognized
        }
    }
}

#[derive(Debug)]
pub enum HttpVersion {
    V1_0,
    V1_1,
    V2_0,
}

impl HttpVersion {
    pub fn as_str(&self) -> &str {
        match self {
            HttpVersion::V1_0 => "HTTP/1.0",
            HttpVersion::V1_1 => "HTTP/1.1",
            HttpVersion::V2_0 => "HTTP/2.0",
        }
    }
}



impl HttpVersion {
    pub fn from_str(version: &str) -> HttpVersion {
        match version {
            "HTTP/1.0" => HttpVersion::V1_0,
            "HTTP/1.1" => HttpVersion::V1_1,
            "HTTP/2.0" => HttpVersion::V2_0,
            _ => HttpVersion::V1_0, // Default to HTTP/1.1
        }
    }
}
#[derive(Debug)]
pub enum StatusCode {
    Ok,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::InternalServerError => "500 Internal Server Error",
        }
    }
}

#[derive(Debug)]
pub struct HttpHeader {
    pub headers: HashMap<String, String>,
}

impl HttpHeader {
    // Parse raw HTTP header lines into a HashMap
    pub fn from_lines(lines: &[&str]) -> HttpHeader {
        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }
        HttpHeader { headers }
    }

    // Get a header value by name (e.g., "Content-Type")
    pub fn get(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }
}

#[derive(Debug)]
pub struct Path {
    pub full_path: String,
    pub query_string: Option<String>, // Optional query parameters
}

impl Path {
    // Parse a full path (e.g., "/index.html?name=John") into a Path struct
    pub fn from_str(path: &str) -> Path {
        if let Some((full_path, query)) = path.split_once('?') {
            Path {
                full_path: full_path.to_string(),
                query_string: Some(query.to_string()),
            }
        } else {
            Path {
                full_path: path.to_string(),
                query_string: None,
            }
        }
    }
}