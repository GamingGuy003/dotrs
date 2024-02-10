use cali::parser::Parser;
use log::error;
use std::process::exit;

mod git;
mod profiles;

fn main() {
    pretty_env_logger::init_timed();
    let mut parser = Parser::new()
        .add_arg("h", "help", "Prints this help prompt", false, None)
        .add_arg("i", "init", "Initializes repo at location", true, None);

    parser.parse().unwrap_or_else(|err| {
        error!("{}", err);
        exit(-1);
    });

    match parser.get_parsed_argument_long("help") {
        (true, _) => parser
            .get_arguments()
            .iter()
            .for_each(|argument| println!("\t{argument}")),
        _ => {}
    }

    match parser.get_parsed_argument_long("init") {
        (true, Some(path)) => {
            println!("{path}")
        },
        _ => {}
    }
}
