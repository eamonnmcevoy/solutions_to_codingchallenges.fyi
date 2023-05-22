# JSON parser

The task is to implement a json parser (and cli) that will scan a given blob and return `0` or `1`.

```
done:
  lexical analysis
todo:
  syntax analysis
```

## Use

`-l` parameter toggles output from the lexical analysis step

```sh
cargo run -- -l test_files/full/pass1.json
```

## JSON structure

JSON (JavaScript Object Notation) is a simple data format based on javascript that is easy to read and write.
JSON is a context free grammar specified [here](https://www.json.org/json-en.html).

### Object
<img src="readme_assets/object.png" style="padding: 10px; background: white" />

### Array
<img src="readme_assets/array.png" style="padding: 10px; background: white" />

### Value
<img src="readme_assets/value.png" style="padding: 10px; background: white" />

### String
<img src="readme_assets/string.png" style="padding: 10px; background: white" />

### Number
<img src="readme_assets/number.png" style="padding: 10px; background: white" />

### Whitespace
<img src="readme_assets/whitespace.png" style="padding: 10px; background: white" />
