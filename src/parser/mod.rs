mod choice;
use crate::stream;

pub fn parse_args(args: Vec<String>) {
    let mut choice: choice::ChoiceGlossary = choice::ChoiceGlossary::new();

    let cmd_name: &str = &args[0];

    let mut args_iter: std::iter::Peekable<std::slice::Iter<'_, String>> = args.iter().peekable();
    let mut arg_option: Option<&String> = args_iter.next();

    while arg_option.is_some() {
        let arg = arg_option.unwrap();

        match arg.as_str() {
            // Explore commands
            arg if !arg.starts_with("-") => {
                match arg {
                    // Main call <exe name>
                    arg if arg == args[0] => {
                        if args.len() == 1 {
                            stream::show_usage(cmd_name);
                            std::process::exit(0);
                        }
                        // skip main call if more arguments...
                    }
                    // Historical information
                    arg if "history".starts_with(arg) => {
                        // TODO - Ticker will consume Info which impl InfoBuilder trait
                        stream::log(
                            std::io::stdout().lock(),
                            "Getting historical ticker info.....",
                            choice.verbose.enable,
                        );
                        break;
                    }
                    // Last command should be ticker symbol
                    arg if arg == &args[args.len() - 1] => {
                        stream::log(
                            std::io::stdout().lock(),
                            "Getting basic ticker info.....",
                            choice.verbose.enable,
                        );
                    }
                    // Command not found
                    _ => {
                        stream::log(
                            std::io::stdout().lock(),
                            &format!("{}: invalid command '{}'", cmd_name, arg),
                            true,
                        );
                        stream::show_usage(cmd_name);
                        std::process::exit(0);
                    }
                }
            }
            // Explore choice short and long
            arg if choice.is_choice(arg, "h", "help") => {
                stream::show_help(cmd_name);
                std::process::exit(0);
            }
            arg if choice.is_choice(arg, "n", "nocache") => choice.nocache.enable = true,
            arg if choice.is_choice(arg, "o", "offline") => choice.offline.enable = true,
            arg if choice.is_choice(arg, "q", "quiet") => choice.quiet.enable = true,
            arg if choice.is_choice(arg, "v", "verbose") => choice.verbose.enable = true,
            arg if choice.is_choice(arg, "V", "version") => {
                stream::show_license(cmd_name);
                std::process::exit(0);
            }
            _ => {
                stream::log(
                    std::io::stdout().lock(),
                    &format!("{}: invalid option '{}'", cmd_name, arg),
                    true,
                );
                stream::show_usage(cmd_name);
                std::process::exit(0);
            }
        }
        arg_option = args_iter.next();
    }
}
