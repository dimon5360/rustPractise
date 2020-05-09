
use std::error::Error;

use mio::net::{TcpStream, TcpListener};
use mio::{Events, Poll, Ready, PollOpt, Token};
use std::time::Duration;

use std::net::*;
use std::thread;

// Application version v.0.0.4 from 09.05.2020
const APP_VERSION: &'static str = "0.0.4";

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> Result<(), std::io::Error> {
    
    println!("Hello, %username%! Application version v.{}.", APP_VERSION);
    println!("Application started.");
    
    start_tcp_socket(40400);
    Ok(())
}

fn start_tcp_socket(u16_ip_port: u16) -> Result<(), std::io::Error> {
    let s_port = u16_ip_port.to_string();
    let s_ip_addr = format!("{}:{}", "127.0.0.1", s_port);

    let addr: SocketAddr = s_ip_addr.parse().unwrap();
    println!("Server is listening on address {}", s_ip_addr);

    let server = TcpListener::bind(&addr)?;                 // listener from mio::net
       
    // Construct a new `Poll` handle as well as the `Events` we'll store into
    let poll = Poll::new()?;

    // Start listening for incoming connections
    poll.register(&server, 
        SERVER, 
        Ready::readable(),
        PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(1024);
    
    let mut iter_count = 0;

    loop {
        poll.poll(&mut events, None).unwrap();
    
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    iter_count += 1;
                    println!("server iter {}", iter_count);
                    let _ = server.accept();
                }
                CLIENT => {
                    // The server just shuts down the socket, let's just exit
                    // from our event loop.
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}