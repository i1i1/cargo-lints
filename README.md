# cargo-lints

This is an utility for running lints specified from files.

## Installation

Install with:
```sh
$ cargo install --git https://github.com/i1i1/clippy-lints
```

## Example

You should set some lints in `lints.toml` file:

### `$ cat lints.toml`
```toml
deny = [
    'clippy::all',
    'clippy::cargo',
    'clippy::nursery',
    'clippy::pedantic',
]
allow = [
    'clippy::enum_glob_use',
]
```

After that you can run:

#### `$ cargo lint fmt`

For formatting toml file.

#### `$ cargo lint clippy`

For running clippy (you don't have to have `lints.toml` file, it will just run `cargo clippy`).

## Options

On top level you can suply `-f` option:
```sh
$ cargo lints -f custom_lints.toml clippy
```

Also all options after `cargo lints clippy` are forwarded to clippy:

```
$ cargo lints clippy --tests --benches --all-features --all
```

