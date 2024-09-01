// super 키워드로 부모 모듈 참조
use super::method::Method;
// convert 모듈에 정의된 TryFrom 트레이트
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug)]
// pub 키워드로 구조체를 공개하여도 구조체의 필드는 비공개로 유지됨 -> new 연관 함수를 공개
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// Request 타입에 TryFrom 트레이트 구현
// 표준 라이브러리에서 제공하는 TryFrom 트레이트를 구현함으로써
// 컴파일러는 TryInto<Request> for &[u8] 또한 구현한다.
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=asc HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Error 트레이트는 Display + Debug 트레이트 바운드를 가짐
// Error 트레이트를 구현하려면 Display + Debug 트레이트를 구현해야 함
impl Error for ParseError {}
