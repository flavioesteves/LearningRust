use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                            //let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
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
