# lineup

> read/write collection of formatted UTF-8 string items

## About

lineup is a Rust library and cross-platform command-line tool for reading and writinf a list of UTF-8 strings according to specified input and output formats (separator, padding, anchor, etc.)

    $ echo -n 'hi,here,hey' | lineup --in-separator=',' --out-span=4 --out-pad='_' --out-anchor=right --out-line-n=1 --out-line-separator='
    > '; echo
    __hi
    here
    _hey

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

    read/write collection of formatted UTF-8 string items
    
    Usage: lineup [OPTIONS]
    
    Options:
          --in-separator <IN_SEPARATOR>
              IN FORMAT: input item separator, possible values:
                N:   N is fixed number of bytes per item, no explicit item separator; NOTE N must be > 0 and boundary of a UTF-8 code point for each item
                SEP: SEP is a string used to separate items; SEP cannot start with a digit
              
              [default: ,]
    
          --in-line-n <IN_LINE_N>
              IN format, line: number of items per line; if 0 provided all items are on a single line
              
              [default: 0]
    
          --in-line-separator <IN_LINE_SEPARATOR>
              IN format, line: separator string between lines
              
              [default: ]
    
          --out-span <OUT_SPAN>
              OUT format, span: max characters an item would need; shorter representations would be padded with 'pad' and anchored according to 'anchor'; if 0, items will not be padded so 'pad' and 'anchor' are not used
              
              [default: 0]
    
          --out-pad <OUT_PAD>
              OUT format, span: pad character (see 'span')
              
              [default: " "]
    
          --out-anchor <OUT_ANCHOR>
              OUT format, span: anchor items to the left or right when padding is needed (see 'span')
              
              [default: left]
              [possible values: right, left]
    
          --out-separator <OUT_SEPARATOR>
              OUT format: separator string for items within a line
              
              [default: " "]
    
          --out-line-n <OUT_LINE_N>
              OUT format, line: number of items per line; if 0 provided put all items on a single line
              
              [default: 0]
    
          --out-line-separator <OUT_LINE_SEPARATOR>
              OUT format, line: separator string between lines
              
              [default: ]
    
      -h, --help
              Print help information (use `-h` for a summary)
    
      -V, --version
              Print version information

### Input format arguments

These arguments specify how input items are arranged in the input stream:

- item separator:```--in-separator```
- line separator:
    - number of items per line: ```--in-line-n```, 0 disables line separation
    - line separator: ```in-line-separator```

### Output format arguments

These arguments specify how items will be arranged on the output stream:

- span:
    - span size: ```--out-span```, 0 disables span
    - pad character: ```--out-pad```
    - anchor: ```--out-anchor```
- item separator:```--out-separator```
- line separator:
    - number of items per line: ```--out-line-n```, 0 disables line separation
    - line separator: ```out-line-separator```

## License

This project is licensed under the **GNU General Public License v3**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

- [clap](https://github.com/clap-rs/clap)
- [amazing-github-template](https://github.com/dec0dOS/amazing-github-template)
