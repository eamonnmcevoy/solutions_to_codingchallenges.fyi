mod parser;

use std::{fs::{self}};

use clap::{Arg, Command, ArgAction};
use parser::{types::{Token, ScanError}, lexer::Lexer};

struct Arguments<'a> {
    filepath: Option<&'a String>,
    lexer_output: bool,
}

fn cli() -> Command {
    Command::new("jsonp")
        .about("Parse json")
        .after_help("")
        .arg(Arg::new("lexer_output").short('l').action(ArgAction::SetTrue).help("Toggle lexer output"))
        // .arg(Arg::new("lexer_output_format").short('t').default_value("debug").action(ArgAction::SetTrue).help("Lexer output format"))
        .arg(Arg::new("filepath"))
}

fn main() {
    let matches = cli().get_matches();
    let args = parse_args(&matches);

    let file_open_result = fs::read_to_string(args.filepath.unwrap());
    if file_open_result.is_err() {
        println!("Error opening file: {}", file_open_result.unwrap_err());
        return;
    }

    let input = file_open_result.unwrap();
    let lexer = Lexer::new();

    let result = lexer.get_tokens(input.as_str());
    
    if args.lexer_output {
        print_tokens(result);
        return;
    }
}

fn print_tokens(result: Result<Vec<Token>, Vec<ScanError>>) {
    match result {
        Ok(tokens) => {
            for token in tokens.clone() {
                println!("{}", token);
            }
            tokens
                .iter()
                .for_each(|t| print!("{} ", t.token_type.to_string()));
        },
        Err(errors) => {
            for error in errors {
                println!("{}", error);
            }
        }
    }
}

fn parse_args(matches: &clap::ArgMatches) -> Arguments {
    let filepath = matches.try_get_one::<String>("filepath").unwrap_or_default();
    let lexer_output: bool = matches.get_flag("lexer_output");

    return Arguments { 
        filepath: filepath.clone(),
        lexer_output: lexer_output,
    }
}