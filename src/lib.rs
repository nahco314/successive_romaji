mod parse;
mod romaji;

pub use parse::*;

#[cfg(test)]
mod tests {
    use crate::{parse, ParseResult, WritingChar};

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
            ParseResult::Writing(
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
        assert_eq!(res, None);
    }

    #[test]
    fn test_parse_hiragana_with_buf_done() {
        let res =
            parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "jinennjyowotaberuzo").unwrap();

        assert_eq!(
            res,
            ParseResult::Completed(tr(&[
                ("じ", "ji"),
                ("ね", "ne"),
                ("ん", "nn"),
                ("じょ", "jyo"),
                ("を", "wo"),
                ("た", "ta"),
                ("べ", "be"),
                ("る", "ru"),
                ("ぞ", "zo")
            ]))
        )
    }
}
