pub mod color;
pub mod parser;
pub mod price;
pub mod stream;

fn main() {
    // Gather arguments from terminal
    let args: Vec<String> = std::env::args().collect();

    parser::parse_args(args);
}
