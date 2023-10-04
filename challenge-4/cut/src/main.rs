pub mod cut;

use std::fs;

use clap::{Command, Arg};

struct Arguments {
    fields: Vec<usize>,
    delim: char,
    filepath: String,
}

fn cli() -> Command {
    Command::new("cut")
        .about("cut – cut out selected portions of each line of a file")
        .arg(Arg::new("fields").short('f').help("The list specifies fields, separated in the input by the field delimiter character (see the -d option).  Output fields are separated by a single occurrence of the field delimiter character."))
        .arg(Arg::new("delim").short('d').default_value("\t").help("Use delim as the field delimiter character instead of the tab character."))
        .arg(Arg::new("filepath").required(true))
}

fn main() {
    let matches = cli().get_matches();
    let args = parse_args(&matches);

    let file_open_result = fs::read_to_string(args.filepath);
    if file_open_result.is_err() {
        println!("Error opening file: {}", file_open_result.unwrap_err());
        return;
    }
    let input = file_open_result.unwrap();

    let result = cut::cut(input, args.fields, args.delim);
    
    print!("{}\n", result);
}

fn parse_args(matches: &clap::ArgMatches) -> Arguments {
    let fields_str = matches.try_get_one::<String>("fields").unwrap();
    let fields: Vec<usize> = match fields_str {
        None => vec![1],
        _ => fields_str.unwrap().split(",").map(|x| x.parse().unwrap()).collect()
    };

    let delim_str = matches.try_get_one::<String>("delim").unwrap();
    let delim = delim_str.unwrap().chars().nth(0);
    
    let filepath = matches.get_one::<String>("filepath").unwrap();

    return Arguments { 
        fields: fields,
        delim: delim.unwrap(),
        filepath: filepath.clone()
    }
}
