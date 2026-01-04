**celq** is a command-line tool for evaluating Common Expression Language (CEL) expressions. It processes JSON input, performs computations, and outputs results. Think of it as if `jq` supported `CEL`.

## Installation

```bash
cargo install celq
```

## Overview

```bash
CEL expression evaluator

Usage: celq [OPTIONS] <expr|--from-file <FILE>>

Arguments:
  [expr]  CEL expression to evaluate

Options:
  -a, --arg <name:type=value>  Define argument variables, types, and (optional) values. Format: name:type=value
  -b, --boolean                Return a status code based on boolean output true = 0, false = 1, exception = 2
  -n, --null-input             Do not read JSON input from stdin
  -s, --slurp                  Treat all input as a single JSON document Default is to treat each line as separate NLJSON
  -j, --jobs <N>               Parallelism level (number of threads, -1 for all available) [default: 1]
  -S, --sort-keys              Output the fields of each object with the keys in sorted order
  -f, --from-file <FILE>       Read CEL expression from a file
  -h, --help                   Print help
```

## Quick Start

TODO