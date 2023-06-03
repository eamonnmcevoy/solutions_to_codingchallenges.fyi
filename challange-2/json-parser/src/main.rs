mod parser;

use std::fs::{self};

use clap::{Arg, ArgAction, Command};
use parser::{
    lexer::Lexer,
    syntax_analyser::SyntaxAnalyzer,
    types::{Token},
};

struct Arguments<'a> {
    filepath: Option<&'a String>,
    lexer_output: bool,
}

fn cli() -> Command {
    Command::new("jsonp")
        .about("Parse json")
        .after_help("")
        .arg(
            Arg::new("lexer_output")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("Toggle lexer output"),
        )
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

    let get_tokens_result = lexer.get_tokens(input.as_str());

    if get_tokens_result.is_err() {
        println!("{}", get_tokens_result.unwrap_err());
        return;
    }

    let tokens = get_tokens_result.unwrap();

    if args.lexer_output {
        print_tokens(tokens.clone());
    }

    let mut syntax_analyser = SyntaxAnalyzer::new();
    let parse_result = syntax_analyser.parse(tokens.into());

    match parse_result {
        Ok(_) => println!("ok"),
        Err(error) => println!("{}", error),
    }
}

fn print_tokens(tokens: Vec<Token>) {
    for token in tokens.clone() {
        println!("{}", token);
    }
}

fn parse_args(matches: &clap::ArgMatches) -> Arguments {
    let filepath = matches
        .try_get_one::<String>("filepath")
        .unwrap_or_default();
    let lexer_output: bool = matches.get_flag("lexer_output");

    return Arguments {
        filepath: filepath.clone(),
        lexer_output: lexer_output,
    };
}
