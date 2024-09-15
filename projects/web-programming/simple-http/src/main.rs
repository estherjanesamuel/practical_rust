use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io::Read;

use simple_http::http::request::HttpRequest;
use simple_http::http::response::HttpResponse;

fn create_socket() -> SocketAddr {
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    // Read from the stream
    match stream.read(&mut buffer) {
        Ok(_) => {
            // let stringified_buffer = String::from_utf8_lossy(&buffer);
            // print!("HTTP request: \n{}", &stringified_buffer);

            // Parse the incoming HTTP request
            let request = HttpRequest::from_buffer(&buffer);

            // Check if the request is for a specific file by extracting the path
            let file_path = request.path.full_path.clone();
            // Handle the file serving logic based on the file_path
            if !file_path.is_empty() && file_path != "/" {
                // Try to serve the requested file from disk
                match std::fs::read_to_string(&file_path[1..]) {  // Strip the leading '/'
                    Ok(contents) => {
                        let response = HttpResponse::from_file(&file_path, &contents);
                        response.send(stream);
                    }
                    Err(_) => {
                        let response = HttpResponse::not_found(&file_path);
                        response.send(stream);
                    }
                }
            } else {
                // Serve a default response for the root path
                let response = HttpResponse::new("Welcome to the Rust HTTP Server!", "/");
                response.send(stream);
            }
        }
        Err(error) => {
            println!("Failed to read from socket: {}", error);
            // Parse the incoming HTTP request
            let request = HttpRequest::from_buffer(&buffer);
            let response = HttpResponse::internal_server_error(&error.to_string());
            response.send(stream);
        }
    }
}

fn main() -> std::io::Result<()> {

    let socket = create_socket();
    // Bind to an address and port
    let listener = TcpListener::bind(socket)?;
    println!("Server listening on {}", socket);

    // Loop over incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the connection
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Failed to establish a connection: {}", e);
            }
        }
    }
    Ok(())
}