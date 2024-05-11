pub mod parser;
pub mod price;
pub mod utils;

fn main() {
    // Gather arguments from terminal
    let args: Vec<String> = std::env::args().collect();

    _ = parser::parse_args(args);
}
