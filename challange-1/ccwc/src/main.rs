
pub mod wc;

use std::io;

use clap::{Arg, Command, ArgAction};

struct Arguments<'a> {
    byte_count: bool,
    char_count: bool,
    line_count: bool,
    word_count: bool,
    filepath: Option<&'a String>,
}

fn cli() -> Command {
    Command::new("ccwc")
        .about("Count characters, words, and lines in a file. Assumes UTF-8 encoding.")
        .after_help("When an option is specified, wc only reports the information requested by that option.  The order of output always takes the form of line, word, byte, and file name.  The default action is equivalent to specifying the -c, -l and -w options.")
        .arg(Arg::new("byte_count").short('c').action(ArgAction::SetTrue).help("The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option."))
        .arg(Arg::new("char_count").short('m').action(ArgAction::SetTrue).help("The number of lines in each input file is written to the standard output."))
        .arg(Arg::new("line_count").short('l').action(ArgAction::SetTrue).help("The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option."))
        .arg(Arg::new("word_count").short('w').action(ArgAction::SetTrue).help("The number of words in each input file is written to the standard output."))
        .arg(Arg::new("filepath"))
}

fn main() {
    let matches = cli().get_matches();
    let args = parse_args(&matches);

    let result: Result<wc::Counts, Box<dyn std::error::Error>>;
    if args.filepath.is_none() {
        // let mut buffer = String::new();
        // let lines = io::stdin().lines();
        // let b = io::stdin()
        // let buff: String = lines.collect();
	    // io::stdin(). (&mut buffer).unwrap();
        result = wc::process_reader(io::stdin());
    }
    else {
        let str = args.filepath.unwrap();
        result = wc::process_file(str.clone());    
    }
    
    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
    }
    else {
        print_counts(args, result.unwrap());
    }
}

fn parse_args(matches: &clap::ArgMatches) -> Arguments {
    let get_byte_count: bool = matches.get_flag("byte_count");
    let get_char_count: bool = matches.get_flag("char_count");
    let get_line_count: bool = matches.get_flag("line_count");
    let get_word_count: bool = matches.get_flag("word_count");
    let filepath = matches.try_get_one::<String>("filepath").unwrap_or_default();
    
    if !get_byte_count && !get_char_count && !get_line_count && !get_word_count {
        return Arguments {
            byte_count: true,
            char_count: true,
            line_count: true,
            word_count: true,
            filepath: filepath.clone(),
        }
    }

    return Arguments { 
        byte_count: get_byte_count,
        char_count: get_char_count,
        line_count: get_line_count,
        word_count: get_line_count,
        filepath: filepath.clone(),
    }
}

fn print_counts(args: Arguments, counts: wc::Counts) {
    if args.line_count {
        print!("{}  ", counts.line_count);
    }

    if args.word_count {
        print!("{}  ", counts.word_count);
    } 
    
    if args.byte_count {
        print!("{}  ", counts.byte_count);
    } else if args.char_count {
        print!("{}  ", counts.char_count);
    }
    
    if args.filepath.is_some() {
        print!("{}\n", args.filepath.clone().unwrap());
        return;
    }
}
