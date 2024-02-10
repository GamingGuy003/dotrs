use cali::parser::Parser;
use log::error;
use std::process::exit;

mod config;
mod git;
mod profiles;

fn main() {
    pretty_env_logger::init_timed();
    let mut parser = Parser::new()
        .add_arg("h", "help", "Prints this help prompt", false, false, None)
        .add_arg(
            "i",
            "init",
            "Initializes repo at location",
            true,
            false,
            None,
        )
        .add_arg(
            "p",
            "profile",
            "Switches profile to specified",
            true,
            false,
            None,
        )
        .add_arg(
            "c",
            "config",
            "Opens config with editor supplied",
            true,
            true,
            Some("vim".to_owned()),
        );

    parser.parse().unwrap_or_else(|err| {
        error!("{}", err);
        exit(-1);
    });

    match parser.get_parsed_argument_long("help") {
        (true, _) => parser.get_arguments().iter().for_each(|argument| {
            if argument.has_value {
                println!(
                    "\t-{} --{}\t<value>\t\t{}",
                    argument.short, argument.long, argument.description
                )
            } else {
                println!(
                    "\t-{} --{}\t\t\t{}",
                    argument.short, argument.long, argument.description
                )
            }
        }),
        _ => {}
    }

    for parsed_argument in parser.get_parsed_arguments() {
        match parsed_argument {
            a if a.long_matches("init".to_owned()) => {}
            a if a.long_matches("profile".to_owned()) => {}
            a if a.long_matches("config".to_owned()) => {
                let editor = a.value.clone().unwrap_or_else(|| {
                    a.get_default().unwrap_or_else(|| {
                        error!("Failed to get default editor?");
                        exit(-1)
                    })
                });
                println!("Editor {editor}")
            }
            _ => {}
        }
    }
}
