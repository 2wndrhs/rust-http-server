use std::{
    io::{Result as IoResult, Write},
    net::TcpStream,
};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        // self.body의 불변 참조를 얻음
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        // HTTP 응답 메시지를 TCP Stream에 쓰기
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
