// super 키워드로 부모 모듈 참조
use super::method::Method;
// convert 모듈에 정의된 TryFrom 트레이트
use std::convert::TryFrom;

#[derive(Debug)]
// pub 키워드로 구조체를 공개하여도 구조체의 필드는 비공개로 유지됨 -> new 연관 함수를 공개
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    pub fn new(path: String, query_string: Option<String>, method: Method) -> Self {
        Self {
            path,
            query_string,
            method,
        }
    }
}

// Request 타입에 TryFrom 트레이트 구현
// 표준 라이브러리에서 제공하는 TryFrom 트레이트를 구현함으로써
// 컴파일러는 TryInto<Request> for &[u8] 또한 구현한다.
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
