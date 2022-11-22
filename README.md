# lineup

> format a collection of strings

## About

lineup is a Rust library and cross-platform command-line tool for formatting a list of UTF-8 strings according to specified separator, padding, anchor, etc.


    $ echo -n '1234' | lineup --input-separator 1 --output-separator ','; echo
    1,2,3,4
    $ echo -n 'hey,hello,hi' | lineup --span 5 --pad . --anchor right  --line-items 1 --line-separator '
    '; echo
    ..hey
    hello
    ...hi
    $ echo -n 'hey,hello,hi' | lineup --span 4 --pad - --anchor left --output-separator '|'; echo
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

    format a collection of strings
    
    Usage: lineup [OPTIONS]
    
    Options:
      -i, --input-separator <INPUT_SEPARATOR>
              Possible values:
                N:   N is fixed number of bytes per item, no explicit item separator; NOTE N must be > 0 and boundary of a UTF-8 code point for each item
                SEP: SEP is a string used to separate items; SEP cannot start with a digit
              
              [default: ","]
    
      -s, --span <ITEM_SPAN>
              max characters an item would need; shorter represantions would be padded with 'pad' and anchored according to 'anchor'; if 0, items will not be padded so 'pad' and 'anchor' are not used
              
              [default: 0]
    
      -p, --pad <ITEM_PAD>
              pad character (see span)
              
              [default: " "]
    
      -a, --anchor <ITEM_ANCHOR>
              anchor items to the left or right when padding is needed
              
              [default: left]
              [possible values: right, left]
    
      -n, --line-items <ITEMS_PER_LINE>
              number of items per line; if 0 provided put all items on a single line
              
              [default: 0]
    
      -o, --output-separator <OUTPUT_ITEM_SEPARATOR>
              separator string for items within a line
              
              [default: " "]
    
      -l, --line-separator <LINE_SEPARATOR>
              separator string between lines
              
              [default: "\n"]
    
      -h, --help
              Print help information (use `-h` for a summary)
    
      -V, --version
              Print version information


## License

This project is licensed under the **GNU General Public License v3**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

- [clap](https://github.com/clap-rs/clap)
- [amazing-github-template](https://github.com/dec0dOS/amazing-github-template)
