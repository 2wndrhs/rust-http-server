#![allow(dead_code)]

mod http;
mod server;
mod website_handler;

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    println!("public_path: {}", public_path);

    let addr = String::from("127.0.0.1:8080");
    // Server::new 연관함수의 매개변수인 `addr`로 소유권 이동
    // -> Server 구조체 인스턴스인 server로 다시 소유권 이동
    let server = Server::new(addr);
    server.run(WebsiteHandler::new(public_path));
}
