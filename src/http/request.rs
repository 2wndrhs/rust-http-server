// super 키워드로 부모 모듈 참조
use super::method::Method;

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
