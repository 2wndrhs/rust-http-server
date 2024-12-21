use crate::{
    http::{self, Response, StatusCode},
    server::Handler,
};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &http::Request) -> http::Response {
        dbg!(request);
        Response::new(StatusCode::Ok, Some("<h1>It works!</h1>".to_string()))
    }
}
