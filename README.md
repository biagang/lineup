# lineup

> format a collection of strings

## About

lineup is a Rust library and cross-platform command-line tool for formatting a list of UTF-8 strings according to specified separator, padding, anchor, etc.


    $ echo -n 'hey,hello,hi' | lineup --span 5 --pad . --anchor right  --line-items 1 --line-separator '
    '; echo
    ..hey
    hello
    ...hi
    $ echo -n 'hey,hello,hi' | lineup --span 4 --pad - --anchor left --item-separator '|'; echo
    hey-|hello|hi--
    $ 

For more info check
```
lineup --help
```

## Getting Started

### Prerequisites

lineup is cross-platofrm, coded in Rust; you need to have a valid [Rust](https://rustup.rs/) installation.

### Get with cargo

```
cargo install -f lineup
```

### Build from sources

1. clone this repository
2. build with cargo:
```
cargo build --release
```
## Usage

Usage: lineup [OPTIONS]

Options:
  -i, --input-separator INPUT-SEPARATOR
          input item separator [default: ,]

  -s, --span ITEM-SPAN
          max characters an item would need; shorter represantions would be padded with 'pad' and anchored according to 'anchor'; if 0, items will not be padded so 'pad' and 'anchor' are not used [default: 8]

  -p, --pad ITEM-PAD
          pad character (see span) [default: " "]

  -a, --anchor ITEM-ANCHOR
          anchor items to the left or right when padding is needed [default: left] [possible values: right, left]

  -n, --line-items ITEMSPERLINE
          number of items per line; if 0 provided put all items on a single line [default: 0]

  -e, --item-separator ITEMSEPARATOR
          separator string for items within a line [default: " "]

  -l, --line-separator LINESEPARATOR
          separator string between lines [default: "\n"]

  -h, --help
          Print help information

  -V, --version
          Print version information


## License

This project is licensed under the **GNU General Public License v3**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

- [clap](https://github.com/clap-rs/clap)
- [amazing-github-template](https://github.com/dec0dOS/amazing-github-template)
