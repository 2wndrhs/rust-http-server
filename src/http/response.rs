use std::io::{Result as IoResult, Write};

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

    // dyn(동적 디스패치) -> 런타임에 vtable을 참조하여 호출할 메서드가 결정됨
    // vtable을 참조하여 호출할 메서드를 결정하므로 성능 오버헤드가 발생할 수 있음
    // impl(정적 디스패치) -> 컴파일 타임에 호출할 메서드가 결정됨
    // 각 제네릭 인스턴스에 대해 별도의 코드가 생성되므로 바이너리의 크기가 커질 수 있음
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
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
