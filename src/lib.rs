use std::fmt::Display;

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
/// let format = lineup::OutFormatBuilder::new()
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
pub struct OutFormat {
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

#[derive(Clone, Debug, Builder)]
#[builder(derive(Debug))]
pub struct InFormat {
    #[builder(default = "InputItemSeparator::default()")]
    pub item_separator: InputItemSeparator,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InputItemSeparator {
    /// explicit item separator
    Explicit(String),
    /// item fixed byte size, no explicit separator
    ByteCount(usize),
}

impl Default for InputItemSeparator {
    fn default() -> Self {
        Self::Explicit(",".to_string())
    }
}

impl Display for InputItemSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Write all input items as per provided format
///
/// # Examples
///
/// ```
/// let input = ["😊😊", "👶", "💼💼💼"];
/// let format = lineup::OutFormatBuilder::new()
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
    format: OutFormat,
) -> Result<(), std::io::Error>
where
    In: Iterator<Item = &'i str>,
    Out: std::io::Write,
{
    let mut writer = ItemWriter::new(format);
    for item in istream {
        writer.write(item, &mut ostream)?;
    }
    Ok(())
}

enum ItemSeparator {
    None,
    Item,
    Line,
}

/// Write input items as per provided format (see [write])
///
/// [write]: ItemWriter::write
pub struct ItemWriter {
    separator: ItemSeparator,
    fmt: OutFormat,
    items_in_line: usize,
}

impl ItemWriter {
    /// Create a new instance with provided format
    pub fn new(fmt: OutFormat) -> Self {
        Self {
            separator: ItemSeparator::None,
            fmt,
            items_in_line: 0,
        }
    }

    /// Write input item as per provided format
    ///
    /// # Examples
    ///
    /// ```
    /// let input = ["😊😊", "👶", "💼💼💼"];
    /// let format = lineup::OutFormatBuilder::new()
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
    /// let mut writer = lineup::ItemWriter::new(format);
    /// let istream = input.into_iter();
    /// let mut ostream = output.as_mut_slice();
    /// for item in istream {
    ///     writer.write(item, &mut ostream).unwrap();
    /// }
    /// let eof = output.iter().position(|x| *x == 0u8).unwrap_or(output.len());
    /// let output = output.split_at(eof).0;
    /// assert_eq!(String::from_utf8(output.to_vec()).unwrap(), expected);
    /// ```
    ///
    pub fn write<Out: std::io::Write>(
        &mut self,
        item: &str,
        writer: &mut Out,
    ) -> Result<(), std::io::Error> {
        // emit separator from previous input
        let item_separator = self.fmt.item_separator.as_bytes(); //
        let line_separator = self.fmt.line_separator.as_bytes(); // todo: put these as struct members
        match self.separator {
            ItemSeparator::None => {}
            ItemSeparator::Item => {
                writer.write_all(item_separator)?;
            }
            ItemSeparator::Line => {
                writer.write_all(line_separator)?;
            }
        }

        // write (padded) input
        let input_chars = item.chars().count();
        if input_chars < self.fmt.item_span {
            let pad_count = self.fmt.item_span - input_chars;
            let pad = String::from_iter(std::iter::repeat(self.fmt.item_pad).take(pad_count));
            match self.fmt.item_anchor {
                Anchor::Left => {
                    writer.write_all(item.as_bytes())?;
                    writer.write_all(pad.as_bytes())?;
                }
                Anchor::Right => {
                    writer.write_all(pad.as_bytes())?;
                    writer.write_all(item.as_bytes())?;
                }
            };
        } else {
            writer.write_all(item.as_bytes())?;
        }

        // decide on separator for next input
        (self.separator, self.items_in_line) = if self.fmt.items_per_line > 0 {
            if self.items_in_line + 1 < self.fmt.items_per_line {
                (ItemSeparator::Item, self.items_in_line + 1)
            } else {
                (ItemSeparator::Line, 0)
            }
        } else {
            (ItemSeparator::Item, 0)
        };
        Ok(())
    }
}

impl OutFormatBuilder {
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
        let format = OutFormatBuilder::new()
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
        let format = OutFormatBuilder::new()
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
