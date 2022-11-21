use clap::{Parser, ValueEnum};
use std::fmt::Display;
use std::sync::Once;

pub struct Config {
    in_sep: InputItemSeparator,
    out_fmt: lineup::Format,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'i' , long, value_parser = InputItemSeparator::parse, default_value_t = InputItemSeparator::Explicit(",".to_string()), long_help = InputItemSeparator::LONG_HELP)]
    /// input item separator
    input_separator: InputItemSeparator,

    // output format (todo: macro to obtain these fields from Format)
    #[arg(short = 's', long = "span", default_value_t = default_format().item_span )]
    /// max characters an item would need; shorter represantions would be padded with 'pad'
    /// and anchored according to 'anchor';
    /// if 0, items will not be padded so 'pad' and 'anchor' are not used
    item_span: usize,

    #[arg(short = 'p', long = "pad", default_value_t = default_format().item_pad)]
    /// pad character (see span)
    item_pad: char,

    #[arg(short = 'a', long = "anchor", value_enum, default_value_t = default_format().item_anchor.into())]
    /// anchor items to the left or right when padding is needed
    item_anchor: Anchor,

    #[arg(short = 'n', long = "line-items", default_value_t = default_format().items_per_line)]
    /// number of items per line; if 0 provided put all items on a single line
    items_per_line: usize, // 0 means no line separaion

    #[arg(short = 'e', long = "item-separator", default_value_t = default_format().item_separator.clone())]
    /// separator string for items within a line
    item_separator: String,

    #[arg(short = 'l', long = "line-separator", default_value_t = default_format().line_separator.clone())]
    /// separator string between lines
    line_separator: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InputItemSeparator {
    /// explicit item separator
    Explicit(String),
    /// item fixed byte size, no explicit separator
    ByteCount(usize),
}

impl InputItemSeparator {
    const LONG_HELP: &'static str = r#"Possible values:
  N:   N is fixed number of bytes per item, no explicit item separator; NOTE N must be > 0 and boundary of a UTF-8 code point for each item
  SEP: SEP is a string used to separate items; SEP cannot start with a digit"#;

    fn parse(arg: &str) -> Result<Self, String> {
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

impl Display for InputItemSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Explicit(s) => format!("\"{}\"", s),
                Self::ByteCount(n) => format!("{}", n),
            }
        )
    }
}

fn default_format() -> &'static lineup::Format {
    static mut FMT: Option<lineup::Format> = None;
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        FMT = Some(lineup::FormatBuilder::new().build().unwrap());
    });
    unsafe { FMT.as_ref().unwrap() }
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

impl Config {
    pub fn new() -> Self {
        let args = Args::parse();
        Self {
            in_sep: args.input_separator,
            out_fmt: lineup::FormatBuilder::default()
                .item_span(args.item_span)
                .item_pad(args.item_pad)
                .item_anchor(args.item_anchor.into())
                .items_per_line(args.items_per_line)
                .item_separator(args.item_separator)
                .line_separator(args.line_separator)
                .build()
                .unwrap(),
        }
    }

    pub fn in_separator(&self) -> &InputItemSeparator {
        &self.in_sep
    }

    pub fn out_format(&self) -> lineup::Format {
        self.out_fmt.clone()
    }

    pub fn istream(&self) -> impl std::io::Read {
        std::io::stdin()
    }

    pub fn ostream(&self) -> impl std::io::Write {
        std::io::stdout()
    }
}
