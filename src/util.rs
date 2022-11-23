use crate::config;

/// # Panics
///
/// Panics in case of `sep` being `config::InputItemSeparator::ByteCount(count)` with `count` not on a UTF-8 code point boundary, because of calling [split_at]
///
/// [split_at]: str::split_at
pub fn to_iter<'i>(
    istream: &'i str,
    sep: &'i config::InputItemSeparator,
) -> Box<dyn Iterator<Item = &'i str> + 'i> {
    match sep {
        config::InputItemSeparator::Explicit(sep) => {
            Box::new(istream.split_terminator(sep.as_str()))
        }
        config::InputItemSeparator::ByteCount(count) => {
            let mut v = vec![];
            let mut cur = istream;
            while !cur.is_empty() {
                let (chunk, rest) = cur.split_at(std::cmp::min(*count, cur.len()));
                v.push(chunk);
                cur = rest;
            }
            Box::new(v.into_iter())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_iter_explicit() {
        let istream = "1;2a;3bbb;";
        let sep = config::InputItemSeparator::Explicit(";".to_string());
        let mut it = to_iter(istream, &sep);
        assert_eq!(Some("1"), it.next());
        assert_eq!(Some("2a"), it.next());
        assert_eq!(Some("3bbb"), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn to_iter_byte_count_ok() {
        let istream = "a1b2c3d4";
        let sep = config::InputItemSeparator::ByteCount(2);
        let mut it = to_iter(istream, &sep);
        assert_eq!(Some("a1"), it.next());
        assert_eq!(Some("b2"), it.next());
        assert_eq!(Some("c3"), it.next());
        assert_eq!(Some("d4"), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    #[should_panic]
    fn to_iter_byte_count_panic() {
        let istream = "a🍺cd";
        let sep = config::InputItemSeparator::ByteCount(1);
        let _ = to_iter(istream, &sep);
    }
}
