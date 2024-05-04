use std::{
    io::{stdout, Write},
    process,
};

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

fn show_usage(cmd_name: &str) {
    println!(
        "Usage: {}{} [OPTIONS]... [COMMAND] TICKER{}",
        color::FAINT,
        cmd_name,
        color::RESET
    );
    println!("Try '{} --help' for more information.", cmd_name);
}

fn log<T: Write>(mut output: T, msg: &str, verbose: bool) {
    if verbose {
        let _ = writeln!(output, "{}", msg);
    }
}

fn show_help(cmd_name: &str) {
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
        "Usage: {}{} [OPTIONS]... [COMMAND] TICKER{}",
        color::FAINT,
        cmd_name,
        color::RESET
    );
    println!("Example: {} -n 'appl'", cmd_name);
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
        "\t{}-q, --quiet{}\t\tHide log messages",
        color::FAINT,
        color::RESET
    );
    println!(
        "\t{}-o, --offline{}\t\tRun without network requests",
        color::FAINT,
        color::RESET
    );
    println!("\t{}-n, --nocache{}\t\tDo not create a cache file: {}WARNING: will delete cache file if exists{}", color::FAINT, color::RESET, color::RED, color::RESET);
    println!("\t{}-h, --help{}\t\tHelp menu", color::FAINT, color::RESET);
    println!();
    // Commands list
    println!("Commands:");
    println!(
        "\t{}history{}\t\t\tThe historical information of a ticker",
        color::FAINT,
        color::RESET
    );
}

struct Options {
    verbose: bool,
    offline: bool,
    nocache: bool,
    quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            verbose: false,
            offline: false,
            nocache: false,
            quiet: false,
        }
    }
}

fn main() {
    // Gather arguments from terminal
    let args: Vec<String> = std::env::args().collect();

    // Set options
    let mut options: Options = Options::default();

    let mut args_iter: std::iter::Peekable<std::slice::Iter<'_, String>> = args.iter().peekable();
    let mut arg_option: Option<&String> = args_iter.next();
    while arg_option.is_some() {
        // Strip short and long option indicatiors
        let original_arg: String = arg_option.unwrap().to_string();
        let arg: String = arg_option.unwrap().trim_start_matches("-").to_string();

        match arg.as_str() {
            // Explore commands
            arg if !original_arg.starts_with("--") && !original_arg.starts_with("-") => {
                // Main call <exe name>
                if arg == args[0] {
                    if args.len() == 1 {
                        show_usage(&args[0]);
                        process::exit(0);
                    }
                    // skip main call if more arguments...
                }
                // Historical information
                else if "history".starts_with(arg) {
                    // TODO - Ticker will consume Info which impl InfoBuilder trait
                    log(
                        stdout().lock(),
                        "Getting historical ticker info.....",
                        options.verbose,
                    );
                    break;
                }
                // Last command should be ticker symbol
                else if arg == &args[args.len() - 1] {
                    log(
                        stdout().lock(),
                        "Getting basic ticker info.....",
                        options.verbose,
                    );
                }
                // Command not found
                else {
                    log(
                        stdout().lock(),
                        &format!("{}: invalid command '{}'", &args[0], arg),
                        true,
                    );
                    show_usage(&args[0]);
                    process::exit(0);
                }
            }
            // Explore options short and long
            arg if "help".starts_with(arg) => {
                show_help(&args[0]);
                process::exit(0);
            }
            arg if "nocache".starts_with(arg) => options.nocache = true,
            arg if "offline".starts_with(arg) => options.offline = true,
            arg if "quiet".starts_with(arg) => options.quiet = true,
            arg if "verbose".starts_with(arg) => options.verbose = true,
            arg if "version".starts_with(arg) || "V" == arg => {
                log(
                    stdout().lock(),
                    &format!(
                        "{} 0.1.0\nCopyright (C) 2021 Free Software Foundation, Inc.\nLicense GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nWritten by Samson Ayeni",
                        &args[0]
                    ),
                    true
                );
                process::exit(0);
            }
            _ => {
                log(
                    stdout().lock(),
                    &format!("{}: invalid option '{}'", &args[0], arg),
                    true,
                );
                show_usage(&args[0]);
                process::exit(0);
            }
        }
        arg_option = args_iter.next();
    }
}
