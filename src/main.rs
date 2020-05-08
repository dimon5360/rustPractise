use std::error::Error;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Poll, Token};

// Application version v.0.0.3 from 09.05.2020
const APP_VERSION: &'static str = "0.0.3";

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> Result<(), String> {
    // Create a poll instance.
let poll = match Poll::new() {
    Ok(poll) => poll,
    Err(e) => panic!("failed to create Poll instance; err={:?}", e),
};

    Ok(())
}