use std::io::stdout;

pub mod color;
pub mod price;
fn main() {
    let bar: price::Price = price::Price::new(4.03, 10.0, 1.0, 6.0);
    bar.show_ohlc(2);
    bar.show_direction(stdout().lock());
    bar.show_strong(stdout().lock());
}
