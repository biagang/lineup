use clap::{Parser, ValueEnum};
use lineup::{ItemSpan, LineSeparator};

#[derive(Debug)]
pub struct Config {
    in_fmt: lineup::InFormat,
    out_fmt: lineup::OutFormat,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, value_parser = InputItemSeparator::parse, default_value = ",", long_help = InputItemSeparator::LONG_HELP)]
    /// IN format: input item separator
    in_separator: InputItemSeparator,

    #[arg(long, default_value = "0")]
    /// IN format, line: number of items per line; if 0 provided all items are on a single line
    in_line_n: usize, // 0 means no line separaion

    #[arg(long, default_value = "")]
    /// IN format, line: separator string between lines
    in_line_separator: String,

    #[arg(long, default_value = "0")]
    /// OUT format, span: max characters an item would need; shorter representations would be padded with 'pad'
    /// and anchored according to 'anchor';
    /// if 0, items will not be padded so 'pad' and 'anchor' are not used
    out_span: usize,

    #[arg(long, default_value = " ")]
    /// OUT format, span: pad character (see 'span')
    out_pad: char,

    #[arg(long, value_enum, default_value = "left")]
    /// OUT format, span: anchor items to the left or right when padding is needed (see 'span')
    out_anchor: Anchor,

    #[arg(long, default_value = " ")]
    /// OUT format: separator string for items within a line
    out_separator: String,

    #[arg(long, default_value = "0")]
    /// OUT format, line: number of items per line; if 0 provided put all items on a single line
    out_line_n: usize, // 0 means no line separaion

    #[arg(long, default_value = "")]
    /// OUT format, line: separator string between lines
    out_line_separator: String,
}

impl InputItemSeparator {
    pub const LONG_HELP: &'static str = r#"IN FORMAT: input item separator, possible values:
  N:   N is fixed number of bytes per item, no explicit item separator; NOTE N must be > 0 and boundary of a UTF-8 code point for each item
  SEP: SEP is a string used to separate items; SEP cannot start with a digit"#;

    pub fn parse(arg: &str) -> Result<Self, String> {
        if let Ok(char_count) = arg.parse() {
            if char_count > 0 {
                Ok(Self::ByteCount(char_count))
            } else {
                Err("number of bytes per item must be > 0".to_string())
            }
        } else {
            Ok(Self::Explicit(arg.to_string()))
        }
    }
}

impl From<InputItemSeparator> for lineup::ItemSeparator {
    fn from(s: InputItemSeparator) -> Self {
        match s {
            InputItemSeparator::Explicit(e) => lineup::ItemSeparator::Explicit(e),
            InputItemSeparator::ByteCount(b) => lineup::ItemSeparator::ByteCount(b),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum Anchor {
    Right,
    Left,
}

impl From<lineup::Anchor> for Anchor {
    fn from(a: lineup::Anchor) -> Self {
        match a {
            lineup::Anchor::Left => Anchor::Left,
            lineup::Anchor::Right => Anchor::Right,
        }
    }
}

impl From<Anchor> for lineup::Anchor {
    fn from(a: Anchor) -> Self {
        match a {
            Anchor::Left => lineup::Anchor::Left,
            Anchor::Right => lineup::Anchor::Right,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum InputItemSeparator {
    /// explicit item separator
    Explicit(String),
    /// item fixed byte size, no explicit separator
    ByteCount(usize),
}

impl Config {
    pub fn new() -> Self {
        let args = Args::parse();
        Self {
            in_fmt: lineup::InFormatBuilder::default()
                .item_separator(args.in_separator.into())
                .line_separator(Self::line_separator(args.in_line_n, args.in_line_separator))
                .build()
                .unwrap(),
            out_fmt: lineup::OutFormatBuilder::default()
                .span(if args.out_span == 0 {
                    None
                } else {
                    Some(ItemSpan::new(
                        args.out_span,
                        args.out_pad,
                        args.out_anchor.into(),
                    ))
                })
                .line_separator(Self::line_separator(
                    args.out_line_n,
                    args.out_line_separator,
                ))
                .item_separator(args.out_separator)
                .build()
                .unwrap(),
        }
    }

    pub fn in_fmt(&self) -> &lineup::InFormat {
        &self.in_fmt
    }

    pub fn out_format(&self) -> lineup::OutFormat {
        self.out_fmt.clone()
    }

    pub fn istream(&self) -> impl std::io::Read {
        std::io::stdin()
    }

    pub fn ostream(&self) -> impl std::io::Write {
        std::io::stdout()
    }

    fn line_separator(n: usize, sep: String) -> Option<LineSeparator> {
        if n > 0 {
            Some(LineSeparator::new(n, sep))
        } else {
            None
        }
    }
}
