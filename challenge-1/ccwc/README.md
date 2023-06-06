# CCWC: Character, Word, and Line Counter

CCWC is a command line utility written in Rust that counts characters, words, and lines in a file. It assumes UTF-8 encoding.

This is a basic rust implementation of the unix `wc` utility. 
This implementation assumes the input is UTF-8 encoded and ignores the LANG, LC_ALL and LC_CTYPE environment variables.

The tool accepts input as a filepath, or stdin:

```sh
$ cargo run -- test_files/pg123.txt

7137  58159  341833  test_files/pg123.txt
```

```sh
$ cat test_files/pg123.txt | cargo run -- 

7137  58159  341833 
```

Unlike the `wc` tool, this implementation does not support multiple files at once.

```sh
Count characters, words, and lines in a file. Assumes UTF-8 encoding.

Usage: ccwc [OPTIONS] [filepath]

Arguments:
  [filepath]  

Options:
  -c          The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option.
  -m          The number of lines in each input file is written to the standard output.
  -l          The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.
  -w          The number of words in each input file is written to the standard output.
  -h, --help  Print help

When an option is specified, wc only reports the information requested by that option.  The order of output always takes the form of line, word, byte, and file name.  The default action is equivalent to specifying the -c, -l and -w options.
```
