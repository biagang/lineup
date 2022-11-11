#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Anchor {
    Right,
    Left,
}

#[derive(Clone, Debug)]
pub struct Format {
    item_span: usize,
    item_pad: char,
    item_anchor: Anchor,
    items_per_line: usize, // 0 means no line separaion
    item_separator: String,
    line_separator: String,
}

impl Format {
    pub fn new(
        item_span: usize,
        item_pad: char,
        item_anchor: Anchor,
        items_per_line: usize,
        item_separator: String,
        line_separator: String,
    ) -> Self {
        Self {
            item_span,
            item_pad,
            item_anchor,
            items_per_line,
            item_separator,
            line_separator,
        }
    }
}

pub struct FormatBuilder {
    proto: Format,
}

impl FormatBuilder {
    pub fn new() -> Self {
        Self {
            proto: Format::new(
                8,
                ' ',
                Anchor::Left,
                8,
                "\n".to_string(),
                "".to_string(),
            ),
        }
    }
    pub fn build(&self) -> Format {
        self.proto.clone()
    }
    pub fn span(&mut self, span: usize) -> &mut Self {
        self.proto.item_span = span;
        self
    }
    pub fn anchor(&mut self, anchor: Anchor) -> &mut Self {
        self.proto.item_anchor = anchor;
        self
    }
    pub fn padding(&mut self, pad: char) -> &mut Self {
        self.proto.item_pad = pad;
        self
    }
    pub fn item_separator(&mut self, sep: String) -> &mut Self {
        self.proto.item_separator = sep;
        self
    }
    pub fn line_separator(&mut self, sep: String) -> &mut Self {
        self.proto.line_separator = sep;
        self
    }
    pub fn items_per_line(&mut self, n: usize) -> &mut Self {
        self.proto.items_per_line = n;
        self
    }
}

pub fn write<In, Out>(
    mut istream: In,
    mut ostream: Out,
    format: Format,
) -> Result<(), std::io::Error>
where
    In: Iterator<Item = String>,
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
    while let Some(input) = istream.next() {
        // emit separator from previous input
        match separator {
            Separator::None => {}
            Separator::Item => {
                ostream.write(item_separator)?;
            }
            Separator::Line => {
                ostream.write(line_separator)?;
            }
        }

        // write (padded) input
        let pad_count = format.item_span - input.chars().count(); // it'd be good to get
                                                                  // chars.count as we write in
                                                                  // case of Anchor::Left
        let pad = String::from_iter(std::iter::repeat(format.item_pad).take(pad_count));
        let out = match format.item_anchor {
            Anchor::Left => input + pad.as_str(),
            Anchor::Right => pad + input.as_str(),
        };
        ostream.write(out.as_bytes())?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = ["001".to_string(), "01".to_string(), "1".to_string()];
        // let expected = "001_|01__|1___*".to_string();
        let expected = "_001|__01;___1*".to_string();
        let mut output = [0u8; 15];
        output[14] = b'*';
        let format = FormatBuilder::new()
            .span(4)
            .anchor(Anchor::Right)
            .item_separator("|".to_string())
            .line_separator(";".to_string())
            .items_per_line(2)
            .padding('_')
            .build();
        write(input.into_iter(), output.as_mut_slice(), format).unwrap();
        assert_eq!(String::from_utf8(output.to_vec()).unwrap(), expected);
    }
}
