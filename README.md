a basic csv linter

## Installation

Install [rust/cargo](https://rustup.rs/) then install this CLI:

```
cargo install jdx-csv-lint
```

## Usage

```sh-session
$ jdx-csv-lint examples/data/test.csv
CSV error: record 100 (line: 100, byte: 1599): found record with 13 fields, but the previous record has 12 fields
```

See options with `--help`:

```
jdx-csv-lint --help
```
