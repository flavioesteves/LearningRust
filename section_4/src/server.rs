use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn hanle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let _listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match _listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // two ways to do
                            // Request::try_from(&buffer as  &[u8]);
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                    //PRE L55
                                    //    dbg!(request);
                                    //    let html = "<h1> OK 200</h1>";
                                    //    Response::new(StatusCode::Ok, Some(html.to_string()))
                                }
                                Err(e) => {
                                    handler.hanle_bad_request(&e)
                                    //PRE L55
                                    //  println!("Failed to parse a request: {}", e);
                                    //  Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                            //let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    };
                }
                Err(e) => println!("Failed to connect: {}", e),
                // _=> "_" as a pattern will act as catch all
            }
            /*
            let res = _listener.accept();

            if res.is_err() {
                continue;
            }

            //tuple
            let (stream, addr) = res.unwrap();
            */
        }

        // Tuple
        //let tup =  (5, "a", _listener);
    }
}
