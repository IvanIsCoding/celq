## jq vs celq

* [jq](https://github.com/jqlang/jq) uses jqlang. It is a very powerful language, but it requires learning a new syntax. [Meanwhile, celq uses the Common Expression Language (CEL)](https://cel.dev/) which is a very constrained language that tries to have a similar syntax to C and JavaScript.
* jq did not support JSON5, YAML, and TOML when celq was originally written.
* jq uses pretty-print by default. Meanwhile, celq uses compact output by default.
* jqlang programs can be reused in Python with a library of the same name ([jq](https://pypi.org/project/jq/)). CEL programs can be reused in Python with [cel-python](https://pypi.org/project/cel-python/).
* jqlang programs can be reused in Go with [gojq](https://github.com/itchyny/gojq). CEL programs can be reused in Go with [cel-go](https://github.com/google/cel-go)
* Other languages generally can embed jqlang with `libjq`. For CEL, that can be done with [cel-cpp](https://github.com/google/cel-cpp) or CEL ports in their respective languages.
* Until benchmarks come out, assume that jq is faster than celq.
* celq can process JSON Lines in parallel, while jq cannot.
* gojq and celq are memory-safe, while jq is not (unless compiled with [Fil-C](https://fil-c.org/))

## yq vs celq

* [yq](https://github.com/mikefarah/yq/) supports formats like XML and HCL that celq does not.
* yq's syntax is inspired by jqlang. 
* yq can output YAML while celq is focused on JSON.
* [yqlib](https://pkg.go.dev/github.com/mikefarah/yq/v4/pkg/yqlib) can be reused in Golang only.
* yq is less hermetic than celq by default. That can be mitigated with the `--security-disable-env-ops ` and `--security-disable-file-ops` flags

## jaq vs celq

* [jaq](https://github.com/01mf02/jaq) uses jqlang, as it is a Rust clone of `jq`
* jqlang can be reused in Rust with [jaq-std](https://crates.io/crates/jaq-std). CEL can be reused in Rust with [cel-rust](https://crates.io/crates/cel/0.12.0).
* jaq supports formats like XML and CBOR that celq does not support.
* jaq can output formats like XML and YAML, while celq is focused on JSON.
* jaq did not support JSON5 when `celq` was originally written (but this could trivially be added to jaq)
* Until benchmarks come out, assume that jaq is faster than celq
* celq can process JSON Lines in parallel, while jaq cannot (but this could trivially be added to jaq)
* jaq and celq are both memory-safe


## jo vs celq

* [jo](https://github.com/jpmens/jo) is less verbose than celq for creating JSON output from the CLI
* jo doesn't read from the input by default. celq needs the `-n` flag to ignore the input.
* jo can created nested objects by composing calls e.g. `jo github=$(jo user=get-celq repo=homebrew-tap)`. At the moment, celq's arguments don't support maps yet

## gron vs celq

* gron can make HTTP calls by default. celq does not and needs `curl example.com | celq`.
* celq can use the `--greppable` flag to achieve the same behavior as gron
* celq can use the `--from-gron` flag to achieve the same behavior as ungron
* gron has been rewritten in C++, see [fastgron](https://github.com/adamritter/fastgron).
* gron did not support JSON5 when celq was originally written.
* gron sorts the output by default while celq does not.

## jsonnet vs celq

* jsonnet has its own syntax with functions, imports, inheritance. celq on the other hand is more constrained and uses CEL. 
* celq can template simple JSON outputs, especially with the `--from-file` flag. It can substitute jsonnet for simple use cases.
* [jsonnet has a warning against server-side evaluation](https://jsonnet.org/ref/bindings.html#server_side). CEL was invented to prevent that kind of issue.

## jql vs celq

* [jql](https://github.com/yamafaktory/jql) uses its own syntax to query JSON with selectors. celq is closer to how imperative programming languges access JSON fields.
* jql did not support JSON5, YAML, and TOML when celq was originally written.

## DuckDB JSON vs celq

* [DuckDB](https://duckdb.org/docs/stable/data/json/overview) excels at querying JSON with schemas using SQL
* celq and jq excel at querying unstructed JSON
* DuckDB did not support JSON5, YAML, and TOML when celq was originally written (and that is probably a good thing)
