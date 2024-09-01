// http 모듈
mod method;
mod request;

// re-export
// http 모듈 외부에서는 http::Method, http::Request로 사용할 수 있음
pub use method::Method;
pub use request::ParseError;
pub use request::Request;
