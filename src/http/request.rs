// super 키워드로 부모 모듈 참조
use super::method::{Method, MethodError};
use core::str;
// convert 모듈에 정의된 TryFrom 트레이트
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

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
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // Result값에 ? 연산자를 사용하면 Result 값이 Err라면, Err 값이 반환됨
        // ? 연산자는 From 트레이트에 정의되어 있는 from 함수를 호출하여
        // ? 연산자가 받은 에러를 현재 함수의 반환 타입에 정의된 에러 타입(ParseError)으로 변환
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
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

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Error 트레이트는 Display + Debug 트레이트 바운드를 가짐
// Error 트레이트를 구현하려면 Display + Debug 트레이트를 구현해야 함
impl Error for ParseError {}
