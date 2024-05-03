pub mod color;
pub mod price;

fn main_menu(cmd_name: &str) {
    // Usage list
    println!(
        "Usage: {}{} [OPTIONS]... TICKER [COMMAND]{}",
        color::FAINT,
        cmd_name,
        color::RESET
    );
    // Title
    println!("Realtime tool for monitoring stocks, cryptocurrencies,\nand many other financial market");
    println!(
        "Example: {}{} -N 'appl'{}",
        color::FAINT,
        cmd_name,
        color::RESET
    );
    println!();
    // Options list
    println!("Options:");
    println!(
        "\t{}-V, --version{}\t\tShow version info",
        color::FAINT,
        color::RESET
    );
    println!("\t{}-v, --verbose{}\t\tVerbose", color::FAINT, color::RESET);
    println!(
        "\t{}-q, --quiet{}\t\t\tHide log messages",
        color::FAINT,
        color::RESET
    );
    println!(
        "\t{}-o --offline{}\t\tRun without network requests",
        color::FAINT,
        color::RESET
    );
    println!("\t{}-N --nocache{}\t\tDo not create a cache file: {}WARNING: will delete cache file if exists{}", color::FAINT, color::RESET, color::RED, color::RESET);
    println!(
        "\t{}-h, --help{}\t\t\tHelp menu",
        color::FAINT,
        color::RESET
    );
    println!();
    // Commands list
    println!("Commands:");
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            main_menu(&args[0])
        } else {
            main_menu(&args[0])
        }
    }
    
}
