use std::fmt::{Display, Formatter, Result as FmtResult};

// Copy 트레이트를 구현한 타입은 값이 이동할 때
// 소유권이 이동하는 대신 단순히 값이 복사됨
#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // StatusCode를 u16으로 캐스팅하기 위해 StatusCode에 Copy 트레이트를 구현
        write!(f, "{}", *self as u16)
    }
}
