mod huffman;

use std::{
    fs::{self},
    io::{self, Read, Write},
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Compress(CompressArguments),
    Decompress(DecompressArguments),
}

#[derive(Parser,Default,Debug)]
struct CompressArguments {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long, default_value = "compressed.txt")]
    output: String,
}

#[derive(Parser,Default,Debug)]
struct DecompressArguments {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long, default_value = "decompressed.txt")]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Compress(args)) => compress_handler(args),
        Some(Commands::Decompress(args)) => decompress_handler(args),
        None => { println!("No command given"); },
    }
}

fn compress_handler(args: CompressArguments) {
    let input = get_file_contents(args.filepath.clone()).unwrap();
    let compressed = huffman::compress(input);

    write_to_file(args.output.clone(), compressed.clone()).unwrap();
}

fn get_file_contents(filepath: String) -> Result<String, String> {
    let file_open_result = fs::read_to_string(filepath);
    if file_open_result.is_err() {
        return Err(format!("Error opening file: {}", file_open_result.unwrap_err()));
    }
    return Ok(file_open_result.unwrap());
}

fn decompress_handler(args: DecompressArguments) {
    let input = get_file_contents(args.filepath.clone()).unwrap();
    let decompressed = huffman::decompress(input);

    write_to_file(args.output.clone(), decompressed.clone()).unwrap();
}

fn write_to_file(filepath: String, contents: String) -> Result<(), String> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filepath)
        .unwrap();

    file.write_fmt(format_args!("{}",contents)).unwrap();
    file.flush().unwrap();

    Ok(())
}
