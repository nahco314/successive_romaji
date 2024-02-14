mod parse;
mod romaji;

pub use parse::*;

#[cfg(test)]
mod tests {
    use crate::{parse, WritingChar};

    fn tr(v: &[(&str, &str)]) -> Vec<(String, String)> {
        return v
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
    }

    #[test]
    fn test_parse_hiragana_with_buf() {
        let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "zinennjix").unwrap();
        assert_eq!(
            res,
            (
                tr(&[("じ", "zi"), ("ね", "ne"), ("ん", "nn"), ("じ", "ji")]),
                WritingChar::new("ょ", "xyo", "x"),
                tr(&[
                    ("を", "wo"),
                    ("た", "ta"),
                    ("べ", "be"),
                    ("る", "ru"),
                    ("ぞ", "zo")
                ]),
            )
        );
    }

    #[test]
    fn test_parse_hiragana_with_buf_error() {
        let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "zinennjixs");
        assert_eq!(res, None,);
    }
}
