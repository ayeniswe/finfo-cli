use super::color::{FAINT, GREEN, RED, RESET};

fn show_art() {
    println!(
        "{} **** ******  ******  *****        ***  **** ******      ***  ***{}",
        GREEN, RESET
    );
    println!(
        "{} * **** ****  **  **  * ** *       * *  * **** ****    ***\\\\  //***{}",
        GREEN, RESET
    );
    println!(" * *          **  **  * * * *      * *  * *           ***  \\\\//  ***");
    println!(" **** ******  **  **  * *   * *    * *  **** ******  ***    \\\\    ***");
    println!(" ****** ****  **  **  * *    * *   * *  ****** ****  ***   //\\\\   ***");
    println!(" * *          **  **  * *     * *  * *  * *          ***  //  \\\\  ***");
    println!(" * *          **  **  * *      * * * *  * *           ***//    \\\\***",);
    println!(
        "{} * *          **  **  * *       * ** *  * *            ***      ***{}",
        GREEN, RESET
    );
    println!(
        "{} ***          ******  * *        *** *  ***             ***    ***{}",
        GREEN, RESET
    );
}

pub fn log<T: std::io::Write>(mut output: T, msg: &str, verbose: &bool) {
    if *verbose {
        let _ = writeln!(output, "{}", msg);
    }
}

pub fn show_usage(cmd_name: &str) {
    println!(
        "Usage: {}{} [OPTIONS]... [COMMAND] TICKER{}",
        FAINT, cmd_name, RESET
    );
    println!("Try '{} --help' for more information.", cmd_name);
}

pub fn show_help(cmd_name: &str) {
    // Title
    println!();
    show_art();
    println!();
    println!(
        "Realtime tool for monitoring stocks, cryptocurrencies,\nand many other financial market"
    );
    println!();
    // Usage list
    println!(
        "Usage: {}{} [OPTIONS]... [COMMAND] TICKER{}",
        FAINT, cmd_name, RESET
    );
    println!("Example: {} -n 'appl'", cmd_name);
    println!();
    // Options list
    println!("Options:");
    println!("\t{}-V, --version{}\t\tShow version info", FAINT, RESET);
    println!("\t{}-v, --verbose{}\t\tVerbose", FAINT, RESET);
    println!("\t{}-q, --quiet{}\t\tHide log messages", FAINT, RESET);
    println!(
        "\t{}-o, --offline{}\t\tRun without network requests",
        FAINT, RESET
    );
    println!("\t{}-n, --nocache{}\t\tDo not create a cache file: {}WARNING: will delete cache file if exists{}", FAINT, RESET, RED, RESET);
    println!("\t{}-h, --help{}\t\tHelp menu", FAINT, RESET);
    println!();
    // Commands list
    println!("Commands:");
    println!(
        "\t{}history{}\t\t\tThe historical information of a ticker",
        FAINT, RESET
    );
}

pub fn show_license(cmd_name: &str) {
    println!("{} 0.1.0\nCopyright (C) 2021 Free Software Foundation, Inc.\nLicense GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nWritten by Samson Ayeni", cmd_name)
}
