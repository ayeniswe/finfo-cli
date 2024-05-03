pub mod color;
pub mod price;

fn finfo_text_art() {
    println!(
        "{} **** ******  ******  *****        ***  **** ******      ***  ***{}",
        color::GREEN,
        color::RESET
    );
    println!(
        "{} * **** ****  **  **  * ** *       * *  * **** ****    ***\\\\  //***{}",
        color::GREEN,
        color::RESET
    );
    println!(" * *          **  **  * * * *      * *  * *           ***  \\\\//  ***");
    println!(" **** ******  **  **  * *   * *    * *  **** ******  ***    \\\\    ***");
    println!(" ****** ****  **  **  * *    * *   * *  ****** ****  ***   //\\\\   ***");
    println!(" * *          **  **  * *     * *  * *  * *          ***  //  \\\\  ***");
    println!(" * *          **  **  * *      * * * *  * *           ***//    \\\\***",);
    println!(
        "{} * *          **  **  * *       * ** *  * *            ***      ***{}",
        color::GREEN,
        color::RESET
    );
    println!(
        "{} ***          ******  * *        *** *  ***             ***    ***{}",
        color::GREEN,
        color::RESET
    );
}

fn main_menu(cmd_name: &str) {
    // Title
    println!();
    finfo_text_art();
    println!();
    println!(
        "Realtime tool for monitoring stocks, cryptocurrencies,\nand many other financial market"
    );
    println!();
    // Usage list
    println!(
        "Usage: {}{} [OPTIONS]... TICKER [COMMAND]{}",
        color::FAINT,
        cmd_name,
        color::RESET
    );
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
    let mut args_iter = args.iter().skip(1); // skip name of the program

    // Set current arg to observe
    let empty_str = String::from("");
    let arg = args_iter.next().unwrap_or(&empty_str);

    if arg == "-h" || arg == "--help" {
        main_menu(&args[0])
    } else {
        main_menu(&args[0])
    }
}
