// server 모듈
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

        // TCP 리스닝 소켓 생성 (클라이언트의 TCP 연결 요청 대기)
        // addr의 소유권이 이동되지 않도록 addr의 참조를 전달
        let listener = TcpListener::bind(&self.addr);

        dbg!(listener);
    }
}
