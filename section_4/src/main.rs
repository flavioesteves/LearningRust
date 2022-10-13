fn main() {
    let string = String::from("125.0.0.1:8080");
    //get the 8080;
    let string_slice = &string[10..];
    let string_borrow: &str = &string;
    let string_literal = "1234";

    //let server = Server::new("127.0.0.1:8080");
    //server.run();
    dbg!(&string);
    dbg!(string_slice);
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server { addr }
    }

    fn run(self) {}
}
