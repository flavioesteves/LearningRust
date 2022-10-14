use std::{io::Read, net::TcpListener};

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
