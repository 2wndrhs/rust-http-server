struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("{}", self.addr);
    }
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
#[derive(Debug)]
struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

fn main() {
    let addr = String::from("127.0.0.1:8080");
    // Server::new 연관함수의 매개변수인 `addr`로 소유권 이동
    // -> Server 구조체 인스턴스인 server로 다시 소유권 이동
    let server = Server::new(addr);
    server.run();

    let request = Request {
        path: String::from("/index.html"),
        query_string: Some(String::from("?name=value")),
        method: Method::GET,
    };

    match request.query_string {
        Some(query_string) => println!("{}", query_string),
        None => println!("No query string"),
    }
}
