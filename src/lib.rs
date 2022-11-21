#[macro_use]
extern crate derive_builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Anchor type for items when padding is needed
pub enum Anchor {
    /// Anchor items to the right
    Right,
    /// Anchor items to the left
    Left,
}

#[derive(Clone, Debug, Builder)]
#[builder(derive(Debug))]
/// Output format
///
/// conveniently FormatBuilder struct can be used for construction:
///
/// # Examples
///
/// ```
/// let format = lineup::FormatBuilder::new()
///     .item_span(4)
///     .item_anchor(lineup::Anchor::Right)
///     .item_separator("|".to_string())
///     .line_separator(";".to_string())
///     .items_per_line(2)
///     .item_pad('_')
///     .build()
///     .unwrap();
/// ```
///
pub struct Format {
    #[builder(default = "0")]
    /// Max characters an item would need; shorter represantions would be padded with [item_pad]
    /// and anchored according to [item_anchor];
    /// if 0, items will not be padded so [item_pad] and [item_anchor] are not used
    ///
    /// [item_pad]: crate::Format::item_pad
    /// [item_anchor]: crate::Format::item_anchor
    pub item_span: usize,
    #[builder(default = "' '")]
    /// Pad character to use for items whose length is less than [item_span]
    ///
    /// [item_span]: crate::Format::item_span
    pub item_pad: char,
    #[builder(default = "Anchor::Left")]
    /// Anchor type for items when padding is needed (see [item_span])
    ///
    /// [item_span]: crate::Format::item_span
    pub item_anchor: Anchor,
    #[builder(default = "0")]
    /// Number of items per line; if 0, all items will be put on a single line
    pub items_per_line: usize,
    #[builder(default = "String::from(\" \")")]
    /// Separator for items within a line
    pub item_separator: String,
    #[builder(default = "String::from(\"\n\")")]
    /// Separator between lines
    pub line_separator: String,
}
/// Write input items as per provided format
///
/// # Examples
///
/// ```
/// let input = ["😊😊", "👶", "💼💼💼"];
/// let format = lineup::FormatBuilder::new()
///     .item_span(4)
///     .item_anchor(lineup::Anchor::Right)
///     .item_separator("🖖".to_string())
///     .line_separator("🔩\n".to_string())
///     .items_per_line(2)
///     .item_pad('👉')
///     .build()
///     .unwrap();
/// let expected = "👉👉😊😊🖖👉👉👉👶🔩\n👉💼💼💼";
/// let mut output = vec![0u8; 100 ];
/// lineup::write(input.into_iter(), output.as_mut_slice(), format).unwrap();
/// let eof = output.iter().position(|x| *x == 0u8).unwrap_or(output.len());
/// let output = output.split_at(eof).0;
/// assert_eq!(String::from_utf8(output.to_vec()).unwrap(), expected);
/// ```
///
pub fn write<'i, In, Out>(
    istream: In,
    mut ostream: Out,
    format: Format,
) -> Result<(), std::io::Error>
where
    In: Iterator<Item = &'i str>,
    Out: std::io::Write,
{
    enum Separator {
        None,
        Item,
        Line,
    }
    let mut items_in_line = 0usize;
    let mut separator = Separator::None;
    let item_separator = format.item_separator.as_bytes();
    let line_separator = format.line_separator.as_bytes();
    for input in istream {
        // emit separator from previous input
        match separator {
            Separator::None => {}
            Separator::Item => {
                ostream.write_all(item_separator)?;
            }
            Separator::Line => {
                ostream.write_all(line_separator)?;
            }
        }

        // write (padded) input
        let input_chars = input.chars().count();
        if input_chars < format.item_span {
            let pad_count = format.item_span - input_chars;
            let pad = String::from_iter(std::iter::repeat(format.item_pad).take(pad_count));
            match format.item_anchor {
                Anchor::Left => {
                    ostream.write_all(input.as_bytes())?;
                    ostream.write_all(pad.as_bytes())?;
                }
                Anchor::Right => {
                    ostream.write_all(pad.as_bytes())?;
                    ostream.write_all(input.as_bytes())?;
                }
            };
        } else {
            ostream.write_all(input.as_bytes())?;
        }

        // decide on separator for next input
        (separator, items_in_line) = if format.items_per_line > 0 {
            if items_in_line + 1 < format.items_per_line {
                (Separator::Item, items_in_line + 1)
            } else {
                (Separator::Line, 0)
            }
        } else {
            (Separator::Item, 0)
        };
    }
    Ok(())
}

impl FormatBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ["001", "01", "1"];
        let expected = "_001|__01;___1*";
        let mut output = [0u8; 15];
        output[14] = b'*';
        let format = FormatBuilder::new()
            .item_span(4)
            .item_anchor(Anchor::Right)
            .item_separator("|".to_string())
            .line_separator(";".to_string())
            .items_per_line(2)
            .item_pad('_')
            .build()
            .unwrap();
        write(input.into_iter(), output.as_mut_slice(), format).unwrap();
        assert_eq!(String::from_utf8(output.to_vec()).unwrap(), expected);
    }

    #[test]
    fn example() {
        let input = ["😊😊", "👶", "💼💼💼"];
        let format = FormatBuilder::new()
            .item_span(4)
            .item_anchor(Anchor::Right)
            .item_separator("🖖".to_string())
            .line_separator("🔩\n".to_string())
            .items_per_line(2)
            .item_pad('👉')
            .build()
            .unwrap();
        let expected = "👉👉😊😊🖖👉👉👉👶🔩\n👉💼💼💼";
        let mut output = vec![0u8; 100];
        write(input.into_iter(), output.as_mut_slice(), format).unwrap();
        let eof = output
            .iter()
            .position(|x| *x == 0u8)
            .unwrap_or(output.len());
        let output = output.split_at(eof).0;
        assert_eq!(String::from_utf8(output.to_vec()).unwrap(), expected);
    }
}
