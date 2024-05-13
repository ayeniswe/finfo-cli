use http::Connection;
pub mod http;
pub mod parser;
pub mod price;
pub mod utils;

fn main() {
    // Gather arguments from terminal
    let args: Vec<String> = std::env::args().collect();
    let test = http::HttpConnection::new("https://httpbin.org".to_string());
    test.get();
    // _ = parser::parse_args(args);
}
