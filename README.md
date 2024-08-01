a basic csv linter

## Installation

Install [rust/cargo](https://rustup.rs/) then install this CLI:

```
cargo install jdx-csv-lint
```

## Usage

To do a basic run with minimal checks:

```sh-session
$ jdx-csv-lint examples/data/bad.csv
CSV error: record 100 (line: 100, byte: 1599): found record with 13 fields, but the previous record has 12 fields
```

To do a run with all checks enabled:

```sh-session
$ jdx-csv-lint --all-checks examples/data/bad_email.csv
[ERROR jdx_csv_lint::linter] Parse error: examples/data/bad_email.csv[4]: (3,INVALID@INVALID@INVALID.INVALID,foo1@INVALID.INVALID) Invalid email address: INVALID@INVALID@INVALID.INVALID
[ERROR jdx_csv_lint::linter] Parse error: examples/data/bad_email.csv[5]: (4,foo2@bar.com,INVALID@INVALID@INVALID.INVALID) Invalid email address: INVALID@INVALID@INVALID.INVALID
[ERROR jdx_csv_lint] examples/data/bad_email.csv is invalid
```

See options with `--help`:

```
jdx-csv-lint --help
```

## Checks

Enable specific checks with `--checks`:

```sh-session
$ jdx-csv-lint --checks email examples/data/good.csv


### `email`
