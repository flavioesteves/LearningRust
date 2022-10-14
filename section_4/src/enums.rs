fn main() {
    let _get = Method::GET("abcd".to_string());
    let _delete = Method::DELETE(100);
    let _post = Method::POST;
    let _put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET(String),
    DELETE(u64),
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

/*
Http Request
Get /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
