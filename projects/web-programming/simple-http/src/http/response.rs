use std::fs;
use std::io::Write;
use std::net::TcpStream;

use super::internal::{HttpVersion, StatusCode};

fn get_content_type(file_path: &str) -> String {
    match file_path {
        path if path.ends_with(".html") => "text/html".to_string(),
        path if path.ends_with(".json") => "application/json".to_string(),
        path if path.ends_with(".css") => "text/css".to_string(),
        path if path.ends_with(".js") => "application/javascript".to_string(),
        path if path.ends_with(".png") => "image/png".to_string(),
        path if path.ends_with(".jpg") || path.ends_with(".jpeg") => "image/jpeg".to_string(),
        _ => "text/plain".to_string(),  // Default case for unknown extensions
    }
}
#[derive(Debug)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: StatusCode,
    pub content_type: String,
    pub content_length: usize,
    pub accept_ranges: String,
    pub current_path: String,
    pub body: String,
}



impl HttpResponse {

     // Build a new HTTP response with custom content
     // Constructor for the HTTP response
    pub fn new(body: &str, current_path: &str) -> HttpResponse {
        HttpResponse {
            version: HttpVersion::V1_1,              // Default to HTTP 1.1
            status_code: StatusCode::Ok,              // Default to 200 OK
            content_type: get_content_type(current_path),   // Default content type
            content_length: body.len(),               // Calculate the length of the body
            accept_ranges: "bytes".to_string(),       // Support for byte ranges
            current_path: current_path.to_string(),   // Path that is being served
            body: body.to_string(),
        }
    }

    // Serve a file with dynamic content type
    pub fn from_file(file_path: &str, body: &str) -> HttpResponse {
        HttpResponse {
            version: HttpVersion::V1_1,
            status_code: StatusCode::Ok,
            content_type: get_content_type(file_path),  // Use dynamic content type
            content_length: body.len(),
            accept_ranges: "bytes".to_string(),
            current_path: file_path.to_string(),
            body: body.to_string(),
        }
    }

    // Handle a 404 response
    pub fn not_found(file_path: &str) -> HttpResponse {
        let body = format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <title>404 Not Found</title>
                <style>
                    body {{
                        font-family: Arial, sans-serif;
                        text-align: center;
                        margin-top: 50px;
                    }}
                    h1 {{
                        color: #FF0000;
                    }}
                    p {{
                        color: #555555;
                    }}
                    a {{
                        text-decoration: none;
                        color: #007BFF;
                    }}
                    a:hover {{
                        text-decoration: underline;
                    }}
                </style>
            </head>
            <body>
                <h1>404 Not Found</h1>
                <p>The file <strong>'{}'</strong> could not be found on this server.</p>
                <p><a href="/">Return to Home</a></p>
            </body>
            </html>"#,
            file_path
        );
    
        HttpResponse {
            version: HttpVersion::V1_1,
            status_code: StatusCode::NotFound,
            content_type: "text/html".to_string(), // Content-Type is now text/html
            content_length: body.len(),
            accept_ranges: "none".to_string(),
            current_path: file_path.to_string(),
            body,
        }
    }
    
    // Handle a 500 response
    pub fn internal_server_error(message: &str) -> HttpResponse {
        let body = format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <title>500 Internal Server Error</title>
                <style>
                    body {{
                        font-family: Arial, sans-serif;
                        text-align: center;
                        margin-top: 50px;
                    }}
                    h1 {{
                        color: #FF0000;
                    }}
                    p {{
                        color: #555555;
                    }}
                    a {{
                        text-decoration: none;
                        color: #007BFF;
                    }}
                    a:hover {{
                        text-decoration: underline;
                    }}
                </style>
            </head>
            <body>
                <h1>500 Internal Server Error</h1>
                <p>An error occured : <strong>'{}'</strong> this server could not procces the request</p>
                <p><a href="/">Return to Home</a></p>
            </body>
            </html>"#,
            message
        );
    
        HttpResponse {
            version: HttpVersion::V1_1,
            status_code: StatusCode::NotFound,
            content_type: "text/html".to_string(), // Content-Type is now text/html
            content_length: body.len(),
            accept_ranges: "none".to_string(),
            current_path: "/".to_string(),
            body,
        }
    }

    // Send the HTTP response to the client
    pub fn send(&self, mut stream: TcpStream) {

        let response = format!(
            "{} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccept-Ranges: {}\r\n\r\n{}",
            self.version.as_str(),
            self.status_code.as_str(),
            self.content_type,
            self.content_length,
            self.accept_ranges,
            self.body,
        );

        println!(
            "{:?} \n{:?} {} \naccept-ranges: {} \ncontent-length: {} \n\n{}",
             self, self.version.as_str(), self.status_code.as_str(), self.accept_ranges,self.content_length, self.body
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}