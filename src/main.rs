// for tcp listen
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};

// Application version v.0.0.1 from 28.04.2020
const APP_VERSION: &'static str = "0.0.1";

fn main() {
    println!("Hello, %username%! Application version v.{}.", APP_VERSION);
    println!("Application started.");

    start_tcp_socket(40400);
}

fn handle_connections(mut stream: TcpStream) -> std::io::Result<()>  {
    let mut buffer = String::new();
    loop {
        let nbytes = stream.read_to_string(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        println!("Received {} bytes {}", nbytes, buffer);

        stream.write_all(b"some bytes");
        stream.flush()?;
    }
}

fn start_tcp_socket(u16_ip_port: u16) -> std::io::Result<()> {
    let s_port = u16_ip_port.to_string();
    let s_ip_addr = format!("{}:{}", "127.0.0.1", s_port);
    let listener = TcpListener::bind(s_ip_addr).unwrap();
    print!("Server is listening on address ");
    println!("{}:{}", "127.0.0.1", s_port);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("DEBUG: got connection from {}",
                         stream.peer_addr().unwrap());
                handle_connections(stream);
            }
            Err(e) => { 
                /* close connection */
            }
        }
    }
    Ok(())
}