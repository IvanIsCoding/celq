**celq** is a command-line tool for evaluating [Common Expression Language (CEL)](https://cel.dev/) expressions. It processes JSON input, performs computations, and outputs results. Think of it as if `jq` supported CEL.

## Installation

### Pre-built Binaries

We publish pre-built binaries for Linux, macOS, and Windows in celq's [GitHub Releases page](https://github.com/IvanIsCoding/celq/releases). To install the current version for Linux or macOS, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | bash
```

Notice that the installer tries not to be clever and doesn't modify `$PATH` or overwrite existing files. To specify a destination, use the `--to` flag:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://get-celq.github.io/install.sh | \
    bash -s -- --to DESTINATION
```

See the the [installation guide](`crate::installation_guide`) for more details on the installer such as `--force` to replace existing binaries, `--target` to specify which binary to download, versioned URLs, GitHub tokens, and more.

#### Installing From Source 

If you want to install from source, celq publishes to [crates.io](https://crates.io/crates/celq).

```bash
cargo install celq
```

### Other Methods

See the the [installation guide](`crate::installation_guide`) for installation instructions with Homebrew, PyPI, NPM, binstall, and more.

## Overview

```none
CEL expression evaluator

Usage: celq [OPTIONS] <expr|--from-file <FILE>>

Arguments:
  [expr]  CEL expression to evaluate

Options:
  -a, --arg <name:type=value>  Define argument variables, types, and values. Format: name:type=value. Supported types: int, uint, float, bool, string
  -b, --boolean                Return a status code based on boolean output true = 0, false = 1, exception = 2
  -n, --null-input             Do not read JSON input from stdin
  -s, --slurp                  Treat all input as a single JSON document Default is to treat each line as separate NLJSON
      --from-json5             Parse input as JSON5 instead of JSON
  -j, --jobs <N>               Parallelism level for NDJSON inputs (number of threads, -1 for all available) [default: 1]
  -R, --root-var <ROOT_VAR>    Variable name for the root JSON input [default: this]
  -S, --sort-keys              Output the fields of each object with the keys in sorted order
  -f, --from-file <FILE>       Read CEL expression from a file
  -p, --pretty-print
  -h, --help                   Print help
```

## Quick Start

`celq` reads JSON from the input and lets users process it with CEL:

```bash
echo '["apples", "bananas", "blueberry"]' | celq 'this.filter(s, s.contains("a"))'
# Outputs: ["apples","bananas"]
```

`celq` can also evaluate expressions with arguments, without reading from the input:

```bash
celq -n --arg='fruit:string=apple' 'fruit.contains("a")'
# Outputs: true
```

Closely related formats such as NDJSON and JSON5 are also supported.

## References

- [CEL Language Definition](https://github.com/google/cel-spec/blob/master/doc/langdef.md)
- [cel-rust](https://github.com/cel-rust/cel-rust): the Rust implementation of CEL powering `celq`

## Inspiration

`celq` is heavily inspired by:
1. [jq](https://jqlang.org/): the most popular command-line utility for dealing with JSON
2. [cel-python](https://github.com/cloud-custodian/cel-python): a Python library with a CLI that heavily influenced `celq` (there are discrepancies, however)
3. [jaq](https://github.com/01mf02/jaq): a `jq` clone written in Rust

## Recipes

We provide recipes with concrete examples for `celq`. During the recipes, we might refer to `yfinance.json`:

```json
{
  "chart": {
    "result": [
      {
        "meta": {
          "currency": "USD",
          "symbol": "AAPL",
          "regularMarketTime":1767387600,
          "fullExchangeName": "NasdaqGS",
          "instrumentType": "EQUITY",
          "timezone": "EST",
          "exchangeTimezoneName": "America/New_York",
          "regularMarketPrice": 271.01,
          "regularMarketDayHigh": 277.825,
          "regularMarketDayLow": 269.02,
          "longName": "Apple Inc.",
          "chartPreviousClose": 250.42
        }
      }
    ]
  }
}
```

This file contains the simplified response from the Yahoo Finance Unofficial JSON API.

### Reading Files

By default, `celq` reads from the standard input. To read from a file, use `<` for input redirection:
```bash
celq "this.chart.result[0].meta.symbol" < yfinance.json
```

It's also possibile to pipe the output from `cat`:

```bash
cat yfinance.json | celq "this.chart.result[0].meta.symbol"
```

Both command outputs: `"AAPL"`.

### Writing Files

`celq` writes by default to the standard output. That output can be piped to a file.

For example:
```bash
cat yfinance.json | celq "this.chart.result[0].meta.longName" > out.txt
```

Creates a file `out.txt` with the content `"Apple Inc."`

### Output JSON

`celq` always writes JSON to the standard output. That can become handy for transforming JSON.

Take for example:

```bash
cat yfinance.json | celq '{"symbol": this.chart.result[0].meta.longName, "price": this.chart.result[0].meta.regularMarketPrice}'
```

The command outputs: `{"price":271.01,"symbol":"Apple Inc."}`

Notice that by default `celq` does not guarantee the key order of the output. If you require so, pass the `--sort-keys` option:

```bash
cat yfinance.json | celq --sort-keys '{"symbol": this.chart.result[0].meta.longName, "price": this.chart.result[0].meta.regularMarketPrice}'
```

### Reading CEL from a file

In the previous example, the CEL expression for the JSON became long. Let's say we saved the expression in `stock.cel` with the following contents:

```python
{
  "symbol": this.chart.result[0].meta.longName, 
  "price": this.chart.result[0].meta.regularMarketPrice
}
```

If we pass the `--from-file` argument, we can load the expression and keep the command succint:

```bash
cat yfinance.json | celq --from-file stock.cel
```

### Dealing with NDJSON

`celq` can deal with [Newline-Delimited JSON (NDJSON)](https://web.archive.org/web/20231218162511/https://ndjson.org/). That format is also called [JSON Lines (JSONL)](https://web.archive.org/web/20251130123805/https://jsonlines.org./).

`celq` detects the content of multi-line files. Firstly, it tries to parse the input as a NDJSON where each line is a JSON value. If that fails, we parse the input as a single JSON file.

Take for example the following file, `example.ndjson`:

```ndjson
{"x": 1.5, "y": 2.5}
{"x": 3.5, "y": 4.5}
```

Giving NDJSON as an input will also return NDJSON as an output. If we run the command:

```bash
cat example.ndjson | celq '{"xy": this.x + this.y}'
```

We'll get as the output:

```ndjson
{"xy": 4.0}
{"xy": 8.0}
```

NDJSON input can also be processed in parallel. Passing `-j -1` as an argument will enable multi-threading with all available threads. Passing `-j N` as an argument will enable `N` threads. Each thread works on a separate line of the JSON independently.

### Slurping

`celq` supports slurping, albeit in a more limited way than `jq`. If the `--slurp` flag is passed, each individual line of a NDJSON is treated as if it was an array entry.

For example:

```bash
cat example.json | celq --slurp "this"
```

Outputs: `[{"y":2.5,"x":1.5},{"y":4.5,"x":3.5}]`. In short, it concatenated the input in a single list.

That can be convenient. Let's say we want to access all values of `x`:

```bash
cat example.json | celq --slurp "this.map(t, t.x)"
```

The command outputs: `[1.5,3.5]`.

### Logical Calculator

`celq` can act as a calculator. If the `-n` option is provided, the tool will not read from the standard input. Combined with arguments, specified by `--arg:<VARIABLE_NAME>:<VARIABLE_TYPE>=<VALUE>`, this makes `celq` a logical calculator.

Take for example:

```bash
celq -n --arg="x:bool=true" --arg="y:bool=false" '(x || y) && !(x && y)'
```

The command outputs: `true`.

### Renaming the root variable

In contrast to `jq` and `cel-python`, `celq` names its root variable `this`. The root `.` is an operator for CEL and leads to invalid expressions.

The root variable can be tweaked through the `--root-var` argument:

For example:

```bash
cat yfinance.json | celq --root-var=request "request.chart.result[0].meta.longName"
```

Outputs: `"Apple Inc."`. This feature can be handy when reusing CEL snippets accross different environments, as they will not use `this` as a variable. That becomes particularly useful with the `--from-file` feature.

### Boolean output

Inspired by `cel-python` and `test`, `celq` also supports the boolean output feature.

If the flag `--boolean` or `-b` is passed to `celq`, it will set the return code based on the truthiness of the value:
* `0` if the result is true
* `1` if the result is false
* `2` if there was an error

That can be chained with bash if statements. For example:

```bash
#!/usr/bin/env bash

FRUIT="apple"

celq -n -b --arg="fruit:string=$FRUIT" 'fruit.contains("a")' > /dev/null
rc=$?

if [ "$rc" -eq 0 ]; then
    echo "$FRUIT contains the letter a"
else
    echo "$FRUIT does not contain the letter a"
fi
```

Will print: `apple contains the letter a`.

Note that for NDJSON inputs, `celq` sets the value based on the value of the last JSON in the NDJSON input.

### Chaining

Because `celq` outputs the same format it reads as the input, chains are easy to make. For example:

```bash
cat yfinance.json | \
  celq "this.chart.result[0]" | \
  celq "this.meta.symbol"
```

Also works as a way to output `"AAPL"` in the command, just like in the first example. When combined with arguments and more elaborate scripts, that can make up for data pipelines.

### JSON5 Support

`celq` also supports [JSON5](https://json5.org/), a popular JSON extension among config files. It also indirectly supports [JSONC](https://jsonc.org/), because JSON5 is a superset of JSONC but don't quote me on that.

To enable the JSON5 parser, pass the `--from-json5` flag. For example:

```bash
echo "[1, 2, 3, 4,]" | celq --from-json5 'this.map(x, x*2)'
```

Outputs: `[2,4,6,8]`. If the `--from-json5` flag is not passed, the command will fail because of the trailing comma on the list. JSON5 is more lenient than JSON and allows for trailing commas and comments.

Notice that passing the `--from-json5` clashes with the `--slurp` flag and with the NDJSON detection.

### Pretty Printing

`celq` by default uses a compact output. This is a contrast to `jq` where the compact output is an opt-in with the `-c` flag.

With that being said, `celq` can pretty-print JSON via the `-p` flag:

```bash
echo '{"a": 1, "b": 2}' | celq -p 'this'
```

Outputs:

```none
{
  "a": 1,
  "b": 2
}
```

## Quirks

1. Do not rely on the order of the JSON output, by default it is randomized due to Rust implementation details. If you need ordering, pass `--sort-keys` as an argument
2. If an argument has the same name as the root variable, the root variable wins
3. If an argument is repeated, the last definition wins (e.g. `--arg=x:bool=false --arg=x:bool=true`, `x` will be true)
4. `.` does not work as a root variable name
5. Pretty-printing can break chaining. `celq` is more limited than `jq` when parsing NDJSON, as it relies heavily on the new-line delimiters. If you pipe the output of a `celq -p` to `celq` again and the original input was NDJSON with multiple lines, things will break.
6. Currently, the `--arg` syntax only supports `int`, `bool`, `float`, and `string`. Support for other CEL types will be added in the future.

## Pronunciation

`celq` is pronounced *“selk”* / *“selq”*. Kind of like the word `silk` but with an `e` instead.