#![feature(let_chains)]

use derive_new::new as New;
use std::fmt::Display;

#[macro_use]
extern crate derive_builder;

#[derive(Clone, Debug, Builder)]
#[builder(derive(Debug))]
pub struct InFormat {
    #[builder(default = "ItemSeparator::default()")]
    pub item_separator: ItemSeparator,

    #[builder(default = "None")]
    pub line_separator: Option<LineSeparator>,
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
/// let format = lineup::OutFormatBuilder::default()
///     .span(Some(lineup::ItemSpan::new(4, '_', lineup::Anchor::Right)))
///     .item_separator("|".to_string())
///     .line_separator(Some(lineup::LineSeparator::new(2, ";".to_string())))
///     .build()
///     .unwrap();
/// ```
///
pub struct OutFormat {
    #[builder(default = "None")]
    /// Item Span (see [ItemSpan])
    ///
    /// [ItemSpan]: crate::ItemSpan
    pub span: Option<ItemSpan>,

    #[builder(default = "String::from(\" \")")]
    /// Separator for items within a line
    pub item_separator: String,

    #[builder(default = "None")]
    /// Separator for lines
    pub line_separator: Option<LineSeparator>,
}

#[derive(New, Clone, Copy, Debug, PartialEq, Eq)]
/// Output items span
pub struct ItemSpan {
    /// Max characters an item would need; shorter represantions would be padded with [pad]
    /// and anchored as per [anchor];
    ///
    /// [pad]: crate::ItemSpan::pad
    /// [anchor]: crate::ItemSpan::anchor
    span: usize,

    /// Pad character to use for items whose length is less than [span]
    ///
    /// [span]: crate::ItemSpan::span
    pad: char,

    /// Anchor type for items when padding is needed (see [span])
    ///
    /// [span]: crate::ItemSpan::span
    anchor: Anchor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Anchor type for items when padding is needed
pub enum Anchor {
    /// Anchor items to the right
    Right,
    /// Anchor items to the left
    Left,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ItemSeparator {
    /// explicit item separator
    Explicit(String),
    /// item fixed byte size, no explicit separator
    ByteCount(usize),
}

#[derive(New, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LineSeparator {
    items_per_line: usize,
    line_separator: String,
}

impl Default for ItemSeparator {
    fn default() -> Self {
        Self::Explicit(",".to_string())
    }
}

impl Display for ItemSeparator {
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
/// let format = lineup::OutFormatBuilder::default()
///     .span(Some(lineup::ItemSpan::new(4, '👉', lineup::Anchor::Right)))
///     .item_separator("🖖".to_string())
///     .line_separator(Some(lineup::LineSeparator::new(2, "🔩\n".to_string())))
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

/// Get an iterator over &str items
///
/// # Examples
///
/// separate by byte count for each item
/// ```
/// let input = "aabbccdd";
/// let fmt = lineup::InFormatBuilder::default()
///     .item_separator(lineup::ItemSeparator::ByteCount(2))
///     .build()
///     .unwrap();
/// let mut it = lineup::read(input, fmt);
/// assert_eq!(Some("aa"), it.next());
/// assert_eq!(Some("bb"), it.next());
/// assert_eq!(Some("cc"), it.next());
/// assert_eq!(Some("dd"), it.next());
/// assert_eq!(None, it.next());
/// ```
///
/// separate by explicit string
/// ```
/// let input = "👉👉👉SEP😊😊SEP🖖SEP💼💼💼";
/// let fmt = lineup::InFormatBuilder::default()
///     .item_separator(lineup::ItemSeparator::Explicit("SEP".to_string()))
///     .build()
///     .unwrap();
/// let mut it = lineup::read(input, fmt);
/// assert_eq!(Some("👉👉👉"), it.next());
/// assert_eq!(Some("😊😊"), it.next());
/// assert_eq!(Some("🖖"), it.next());
/// assert_eq!(Some("💼💼💼"), it.next());
/// assert_eq!(None, it.next());
/// ```
///
pub fn read(input: &str, format: InFormat) -> impl Iterator<Item = &str> {
    ItemReader::new(input, format)
}

enum EmittingSeparator {
    None,
    Item,
    Line,
}

/// Write input items as per provided format (see [write])
///
/// [write]: ItemWriter::write
#[derive(New)]
pub struct ItemWriter {
    #[new(value = "EmittingSeparator::None")]
    separator: EmittingSeparator,
    fmt: OutFormat,
    #[new(value = "0")]
    items_in_line: usize,
}

#[derive(New)]
pub struct ItemReader<'i> {
    input: &'i str,
    fmt: InFormat,
    #[new(value = "0")]
    items_in_current_line: usize,
}

impl<'i> ItemReader<'i> {
    pub fn next_item(&mut self, separator: ItemSeparator) -> Option<&'i str> {
        if self.input.is_empty() {
            None
        } else {
            match &separator {
                ItemSeparator::Explicit(separator) => match self.input.split_once(separator) {
                    None => {
                        let last = self.input;
                        self.input = "";
                        Some(last)
                    }
                    Some((item, remainder)) => {
                        self.input = remainder;
                        if item.is_empty() {
                            None
                        } else {
                            Some(item)
                        }
                    }
                },
                ItemSeparator::ByteCount(count) => {
                    if self.input.len() >= *count {
                        let split = self.input.split_at(*count);
                        self.input = split.1;
                        Some(split.0)
                    } else {
                        self.input = "";
                        None
                    }
                }
            }
        }
    }
}

impl<'i> Iterator for ItemReader<'i> {
    type Item = &'i str;
    fn next(&mut self) -> Option<Self::Item> {
        let separator = {
            if let Some(line_separator) = &self.fmt.line_separator {
                if self.items_in_current_line == line_separator.items_per_line - 1 {
                    self.items_in_current_line = 0;
                    ItemSeparator::Explicit(line_separator.line_separator.clone())
                } else {
                    self.items_in_current_line += 1;
                    self.fmt.item_separator.clone()
                }
            } else {
                self.fmt.item_separator.clone()
            }
        };
        self.next_item(separator)
    }
}

impl ItemWriter {
    /// Write input item as per provided format
    ///
    /// # Examples
    ///
    /// ```
    /// let input = ["😊😊", "👶", "💼💼💼"];
    /// let format = lineup::OutFormatBuilder::default()
    ///     .span(Some(lineup::ItemSpan::new(4, '👉', lineup::Anchor::Right)))
    ///     .item_separator("🖖".to_string())
    ///     .line_separator(Some(lineup::LineSeparator::new(2, "🔩\n".to_string())))
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
        match self.separator {
            EmittingSeparator::None => {}
            EmittingSeparator::Item => {
                writer.write_all(self.fmt.item_separator.as_bytes())?;
            }
            EmittingSeparator::Line => {
                writer.write_all(
                    self.fmt
                        .line_separator
                        .as_ref()
                        .unwrap()
                        .line_separator
                        .as_bytes(),
                )?;
            }
        }

        // write (padded) input
        let input_chars = item.chars().count();
        if let Some(span) = self.fmt.span.as_ref() && input_chars < span.span {
            let pad_count = span.span - input_chars;
            let pad = String::from_iter(std::iter::repeat(span.pad).take(pad_count));
            match span.anchor {
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
        (self.separator, self.items_in_line) =
            if let Some(line_separator) = self.fmt.line_separator.as_ref() {
                if self.items_in_line + 1 < line_separator.items_per_line {
                    (EmittingSeparator::Item, self.items_in_line + 1)
                } else {
                    (EmittingSeparator::Line, 0)
                }
            } else {
                (EmittingSeparator::Item, 0)
            };
        Ok(())
    }
}

#[cfg(test)]
mod write_test {
    use super::*;

    #[test]
    fn test() {
        let input = ["001", "01", "1"];
        let expected = "_001|__01;___1*";
        let mut output = [0u8; 15];
        output[14] = b'*';
        let format = OutFormatBuilder::default()
            .span(Some(ItemSpan::new(4, '_', Anchor::Right)))
            .item_separator("|".to_string())
            .line_separator(Some(LineSeparator::new(2, ";".to_string())))
            .build()
            .unwrap();
        write(input.into_iter(), output.as_mut_slice(), format).unwrap();
        assert_eq!(String::from_utf8(output.to_vec()).unwrap(), expected);
    }

    #[test]
    fn example() {
        let input = ["😊😊", "👶", "💼💼💼"];
        let format = OutFormatBuilder::default()
            .span(Some(ItemSpan::new(4, '👉', Anchor::Right)))
            .item_separator("🖖".to_string())
            .line_separator(Some(LineSeparator::new(2, "🔩\n".to_string())))
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

#[cfg(test)]
mod read_test {
    use super::*;

    #[test]
    fn reader_explicit() {
        let input = "a,bb,ccc,,";
        let mut reader = ItemReader::new(
            input,
            InFormatBuilder::default()
                .item_separator(ItemSeparator::Explicit(",".to_string()))
                .build()
                .unwrap(),
        );
        assert_eq!(Some("a"), reader.next());
        assert_eq!(Some("bb"), reader.next());
        assert_eq!(Some("ccc"), reader.next());
        assert_eq!(None, reader.next());
    }

    #[test]
    fn reader_byte_count() {
        let input = "aaaabbbbccccddd";
        let mut reader = ItemReader::new(
            input,
            InFormatBuilder::default()
                .item_separator(ItemSeparator::ByteCount(4))
                .build()
                .unwrap(),
        );
        assert_eq!(Some("aaaa"), reader.next());
        assert_eq!(Some("bbbb"), reader.next());
        assert_eq!(Some("cccc"), reader.next());
        assert_eq!(None, reader.next());
    }

    #[test]
    fn reader_explicit_multiline() {
        let input = "aa,vvv,cccc,\nd,ee\n,a\n";
        let mut reader = ItemReader::new(
            input,
            InFormatBuilder::default()
                .item_separator(ItemSeparator::Explicit(",".to_string()))
                .line_separator(Some(LineSeparator {
                    items_per_line: 3,
                    line_separator: "\n".to_string(),
                }))
                .build()
                .unwrap(),
        );
        assert_eq!(Some("aa"), reader.next());
        assert_eq!(Some("vvv"), reader.next());
        assert_eq!(Some("cccc,"), reader.next());
        assert_eq!(Some("d"), reader.next());
        assert_eq!(Some("ee\n"), reader.next());
        assert_eq!(Some("a"), reader.next());
        assert_eq!(None, reader.next());
    }

    #[test]
    fn reader_byte_count_multiline() {
        let input = "aavvcc;ddeebb;";
        let mut reader = ItemReader::new(
            input,
            InFormatBuilder::default()
                .item_separator(ItemSeparator::ByteCount(2))
                .line_separator(Some(LineSeparator {
                    items_per_line: 3,
                    line_separator: ";".to_string(),
                }))
                .build()
                .unwrap(),
        );
        assert_eq!(Some("aa"), reader.next());
        assert_eq!(Some("vv"), reader.next());
        assert_eq!(Some("cc"), reader.next());
        assert_eq!(Some("dd"), reader.next());
        assert_eq!(Some("ee"), reader.next());
        assert_eq!(Some("bb"), reader.next());
        assert_eq!(None, reader.next());
    }

    #[test]
    fn example_byte_count() {
        let input = "aabbccdd";
        let fmt = InFormatBuilder::default()
            .item_separator(ItemSeparator::ByteCount(2))
            .build()
            .unwrap();
        let mut it = read(input, fmt);
        assert_eq!(Some("aa"), it.next());
        assert_eq!(Some("bb"), it.next());
        assert_eq!(Some("cc"), it.next());
        assert_eq!(Some("dd"), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn example_explicit() {
        let input = "👉👉👉SEP😊😊SEP🖖SEP💼💼💼";
        let fmt = InFormatBuilder::default()
            .item_separator(ItemSeparator::Explicit("SEP".to_string()))
            .build()
            .unwrap();
        let mut it = read(input, fmt);
        assert_eq!(Some("👉👉👉"), it.next());
        assert_eq!(Some("😊😊"), it.next());
        assert_eq!(Some("🖖"), it.next());
        assert_eq!(Some("💼💼💼"), it.next());
        assert_eq!(None, it.next());
    }
}
