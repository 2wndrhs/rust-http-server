// server 모듈
use crate::http::{ParseError, Request, Response, StatusCode};
use std::{convert::TryFrom, io::Read, net::TcpListener};
pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, Some(e.message().to_string()))
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        // TCP 리스닝 소켓 생성 (특정 IP 주소, 포트에 바인딩)
        // addr의 소유권이 이동되지 않도록 addr의 참조를 전달
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // 클라이언트 연결이 들어올 때까지 대기(스레드 블로킹)
            // 연결이 들어오면 연결을 수락하고 TCP Stream과 클라이언트 소켓의 주소를 튜플로 반환(Connection Socket)
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("new client: {:?}", addr);

                    let mut buffer = [0; 1024];
                    // TCP Stream의 read 메서드는 io::Read 트레이트를 구현
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(error) => handler.handle_bad_request(&error),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(error) => println!("could not read from client: {:?}", error),
                    }
                }
                Err(error) => println!("could not get client: {:?}", error),
            }
        }
    }
}
