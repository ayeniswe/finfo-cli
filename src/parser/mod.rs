//! This crate provides functionality for parsing commmands and options in
//! the cli.
//!
//! Adding new options should be place in the `ChoiceGlossary` and
//! commands in the `CommandGlossary` (NOTE: not implemented!)

mod choice;
mod glossary;
use self::glossary::Glossary;
use crate::utils;

pub fn parse_args(args: Vec<String>) -> Result<(), String> {
    let mut choice = glossary::ChoiceGlossary::new();

    let cmd_name: &str = &args[0];

    let mut args_iter: std::iter::Peekable<std::slice::Iter<'_, String>> = args.iter().peekable();
    let mut arg_option: Option<&String> = args_iter.next();

    while arg_option.is_some() {
        let arg: &String = arg_option.unwrap();

        // Search glossary for ambigous matches
        let ambiguous_choices = choice.search(arg);
        if ambiguous_choices.len() > 1 {
            print!("{}: option {} is ambiguous; possibilities: ", cmd_name, arg);
            for choice in ambiguous_choices {
                print!("'{}' ", choice);
            }
            println!();

            utils::stream::show_usage(cmd_name);

            std::process::exit(0);
        }

        match arg {
            // Explore commands
            arg if !arg.starts_with("-") => {
                match arg {
                    // Main call <exe name>
                    arg if arg == cmd_name => {
                        if args.len() == 1 {
                            utils::stream::show_usage(cmd_name);
                            std::process::exit(0);
                        }
                        // skip main call if more arguments...
                    }
                    // Historical information
                    arg if "history".starts_with(arg) => {
                        // TODO - Ticker will consume Info which impl InfoBuilder trait
                        utils::stream::log(
                            std::io::stdout().lock(),
                            "Getting historical ticker info.....",
                            &choice.verbose.get_state()?,
                        );
                        break;
                    }
                    // Last command should be ticker symbol
                    arg if arg == &args[args.len() - 1] => {
                        utils::stream::log(
                            std::io::stdout().lock(),
                            "Getting basic ticker info.....",
                            &choice.verbose.get_state()?,
                        );
                    }
                    // Command not found
                    _ => {
                        utils::stream::log(
                            std::io::stdout().lock(),
                            &format!("{}: invalid command '{}'", cmd_name, arg),
                            &true,
                        );
                        utils::stream::show_usage(cmd_name);
                        std::process::exit(0);
                    }
                }
            }
            // Explore choice short and long
            arg if choice.is_choice(arg, &choice.help) => {
                utils::stream::show_help(cmd_name);
                std::process::exit(0);
            }
            arg if choice.is_choice(arg, &choice.nocache) => choice.nocache.enable()?,
            arg if choice.is_choice(arg, &choice.offline) => choice.offline.enable()?,
            arg if choice.is_choice(arg, &choice.quiet) => choice.quiet.enable()?,
            arg if choice.is_choice(arg, &choice.verbose) => choice.verbose.enable()?,
            arg if choice.is_choice(arg, &choice.version) => {
                utils::stream::show_license(cmd_name);
                std::process::exit(0);
            }
            _ => {
                utils::stream::log(
                    std::io::stdout().lock(),
                    &format!("{}: invalid option '{}'", cmd_name, arg),
                    &true,
                );
                utils::stream::show_usage(cmd_name);
                std::process::exit(0);
            }
        };
        arg_option = args_iter.next();
    }

    Ok(())
}
