use std::net::{TcpListener, TcpStream, Shutdown};

// Application version v.0.0.1 from 28.04.2020
const APP_VERSION: &'static str = "0.0.1";

fn main() {
    println!("Hello, %username%! Application version v.{}.", APP_VERSION);
    println!("Application started.");

    start_tcp_socket(40400);
}

fn start_tcp_socket(u16_ip_port: u16) {
    let s_port = u16_ip_port.to_string();
    let s_ip_addr = format!("{}:{}", "127.0.0.1", s_port);
    let listener = TcpListener::bind(s_ip_addr).unwrap();
    print!("Server is listening on address ");
    println!("{}:{}", "127.0.0.1", s_port);
}