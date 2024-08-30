mod http;
mod server;

use server::Server;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    // Server::new 연관함수의 매개변수인 `addr`로 소유권 이동
    // -> Server 구조체 인스턴스인 server로 다시 소유권 이동
    let server = Server::new(addr);
    server.run();

    let request = http::Request::new(
        String::from("/index.html"),
        Some(String::from("?name=value")),
        http::Method::GET,
    );

    dbg!(request);
}
