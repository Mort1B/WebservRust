use crate::http::{request, status_codes, Response};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        return Server { addr };
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let res = listener.accept();

            match res {
                Ok((mut stream, addr)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let data_recieved = String::from_utf8_lossy(&buffer);
                            println!("Recieved data {}", data_recieved);

                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    println!("Request = {:?}", req);

                                    let res = Response::new(
                                        status_codes::StatusCode::Ok,
                                        Some("<h1>Hellloooo<h1>".to_string()),
                                    );
                                    res.send(&mut stream);
                                }
                                Err(_) => {
                                    let res = Response::new(status_codes::StatusCode::Ok, None);
                                    res.send(&mut stream);
                                }
                            }
                        }
                        Err(e) => println!("Failed to read from connection {}", e),
                    }
                }
                Err(e) => print!("Error {}", e),
            }
        }
    }
}
