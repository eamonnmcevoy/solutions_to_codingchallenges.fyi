# cut tool

This challenge is to build your own version of the Unix command line tool cut!

---

The tool implements the `-f` functionality along with the `-d` flag.

```
cut – cut out selected portions of each line of a file

Usage: cut [OPTIONS] <filepath>

Arguments:
  <filepath>  

Options:
  -f <fields>      The list specifies fields, separated in the input by the field delimiter character (see the -d option).  Output fields are separated by a single occurrence of the field delimiter character.
  -d <delim>       Use delim as the field delimiter character instead of the tab character. [default: "\t"]
  -h, --help       Print help
  ```

## Example usage

test files can be found under ./tests/files/

```bash
➜  cut git:(main) ✗ cargo run -- -f 1,2 -d, ./tests/files/fourchords.csv | head -n 5
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/cut -f 1,2 -d, ./tests/files/fourchords.csv`
Song title,Artist
"10000 Reasons (Bless the Lord)",Matt Redman and Jonas Myrin
"20 Good Reasons",Thirsty Merc
"Adore You",Harry Styles
"Africa",Toto
```

```bash
➜  cut git:(main) ✗ cargo run -- -f 1,2  ./tests/files/sample.tsv | head -n 5
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/cut -f 1,2 ./tests/files/sample.tsv`
f0      f1
0       1
5       6
10      11
15      16
```