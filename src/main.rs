use mio::net::*;
use mio::*;
use std::collections::HashMap;
use std::io::*;


/* User variables ========================================================== */

// Application version v.0.0.5 from 14.05.2020
const APP_VERSION: &'static str = "0.0.5";

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

// Size of events collection
const EVENT_CAPACITY: usize = 1024;
const BUFFER_SIZE: usize = 1024;

static RESPONSE: &str = "HTTP/1.1 200 OK
Content-Type: text/html
Connection: keep-alive
Content-Length: 6
hello
";

/* User code =============================================================== */


/***************************************
 * @brief Main method in application
 */
fn is_double_crnl(window: &[u8]) -> bool {
    window.len() >= 4 &&
        (window[0] == '\r' as u8) &&
        (window[1] == '\n' as u8) &&
        (window[2] == '\r' as u8) &&
        (window[3] == '\n' as u8)
}

/***************************************
 * @brief Main method in application
 */
fn main() {
    
    println!("Hello, %username%! Application version v.{}.", APP_VERSION);
    println!("Application started.");
    
    println!("{}", start_tcp_socket("127.0.0.1:40400".to_string()).is_ok());

}

/***************************************
 * @brief Start TCP socket
 */
fn start_tcp_socket(address: String) -> std::io::Result<(TcpStream, std::net::SocketAddr)> {

    println!("Server is listening on address {}", address);

    // listener from mio::net
    let server = TcpListener::bind(&address.parse().unwrap())?;  
       
    // Construct a new `Poll` handle as well as the `Events` we'll store into
    let poll = Poll::new()?;

    // Start listening for incoming connections
    poll.register(
        &server, 
        Token(0),
        Ready::readable(),
        PollOpt::edge()).unwrap();

    // sercer acceptions counter
    let mut counter: usize = 0;
    // map to keep sockets info
    let mut sockets: HashMap<Token, TcpStream> = HashMap::new();
    let mut requests: HashMap<Token, Vec<u8>> = HashMap::new();
    // buffer for data exchange with client
    let mut buffer = [0 as u8; BUFFER_SIZE];

    // events keep, capacity is default const  
    let mut events = Events::with_capacity(EVENT_CAPACITY);

    loop {
        poll.poll(&mut events, None).unwrap();
    
        for event in &events {

            // handle the event
            match event.token() {
                SERVER => {
                    loop {
                        match server.accept() {
                            Ok((socket, address)) => {

                                println!("Got connection from {}", address);

                                counter += 1;
                                let token = Token(counter);
            
                                // Register for readable events
                                poll.register(&socket, 
                                    token,
                                    Ready::readable(),
                                    PollOpt::edge()).unwrap();
            
                                sockets.insert(token, socket);
                                requests.insert(token, Vec::with_capacity(192));
                            },
                            Err(ref e) if e.kind() == ErrorKind::WouldBlock =>
                                break,
                            Err(_) =>
                                break
                        }
                    }
                },
                

                token if event.readiness().is_readable() => {
                    // Socket associated with token is ready for reading data from it
                    loop {
                        let read = sockets.get_mut(&token).unwrap().read(&mut buffer);
                        match read {
                            Ok(0) => {
                                sockets.remove(&token);
                                break;
                            },
                            Ok(len) => {
                                println!("Read {} bytes for token {}", len, token.0);
                                let req = requests.get_mut(&token).unwrap();
                                for b in &buffer[0..len] {
                                    req.push(*b);
                                }
                            },
                            Err(ref e) if e.kind() == ErrorKind::WouldBlock => 
                                break,
                            Err(_) => 
                                break
                        }
                    }


                    let ready = requests.get(&token).unwrap()
                        .windows(4)
                        .find(|window| is_double_crnl(*window))
                        .is_some();

                    if ready {
                        let socket = sockets.get(&token).unwrap();
                        poll.reregister(
                            socket,
                            token,
                            Ready::writable(),
                            PollOpt::edge() | PollOpt::oneshot()).unwrap();
                    }
                },
                

                token if event.readiness().is_writable() => {
                    requests.get_mut(&token).unwrap().clear();
                    // send response
                    println!("Respones : {}", RESPONSE);
                    sockets.get_mut(&token).unwrap().write_all(RESPONSE.as_bytes()).unwrap();
                    
                    // Re-use existing connection ("keep-alive") - switch back to reading
                    poll.reregister(
                        sockets.get(&token).unwrap(),
                        token,
                        Ready::readable(),
                        PollOpt::edge()).unwrap();
                },
                _ => unreachable!()
            }
        }
    }
}